mod configuration;
use configuration::Configuration;

mod defs;
mod deh;
mod game;
mod info;

mod misc;
use misc::args;
use misc::lprint::OutputLevel;

mod sounds;
mod system;
mod think;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use std::{env, fs};

lazy_static! {
    static ref ARGS: RwLock<Vec<String>> = RwLock::new(vec![]);
    pub static ref FORCE_OLD_BSP: RwLock<bool> = RwLock::new(false);
    static ref SAVE_GAME_BASE: RwLock<String> = RwLock::new(String::new());
}

fn init_statics() {
    *ARGS.write() = env::args().collect();
}

pub(crate) fn error<S: AsRef<str>>(why: S) -> ! {
    lprint!(OutputLevel::ERROR, "{}\n", why.as_ref());
    std::process::exit(-1);
}

fn read_configuration() -> Configuration {
    let mut configuration = Configuration::default();

    // TODO

    configuration
}

fn read_args(configuration: &mut Configuration) {
    args::check_arg_conflicts();
}

fn print_version() {
    lprint!(OutputLevel::INFO, "{}\n", system::version_string());
}

fn main() {
    init_statics();

    let mut configuration = read_configuration();
    read_args(&mut configuration);
    print_version();

    let sdl = pre_init_graphics();

    doom_main();
}

fn pre_init_graphics() -> sdl2::Sdl {
    match sdl2::init() {
        Ok(sdl) => sdl,
        Err(e) => {
            error(format!("Could not initialize SDL [{}]", e));
        }
    }
}

fn doom_main() {
    doom_main_setup();

    doom_loop();
}

fn doom_main_setup() {
    setup_console_masks();

    loop {
        let mut rsp_found = false;
        for arg in &*ARGS.read() {
            if arg.starts_with('@') {
                rsp_found = true;
            }
        }
        find_response_file();
        if !rsp_found {
            break;
        }
    }

    if args::check_parm("-forceoldbsp").is_some() {
        *FORCE_OLD_BSP.write() = true;
    }

    deh::build_bex_tables();

    args::handle_loose_files();

    identify_version();
}

fn identify_version() {
    *SAVE_GAME_BASE.write() = env::var("DOOMSAVEDIR").unwrap_or_else(|_| doom_exe_dir());
    if let Some(i) = args::check_parm("-save") {
        if i < ARGS.read().len() - 1 {
            let path = &ARGS.read()[i + 1];
            if let Ok(true) = fs::metadata(path).map(|m| m.is_dir()) {
                *SAVE_GAME_BASE.write() = path.clone();
            } else {
                lprint!(
                    OutputLevel::ERROR,
                    "Error: -save path does not exist. Using {} instead\n",
                    SAVE_GAME_BASE.read()
                );
            }
        }
    }
    normalize_slashes(&mut *SAVE_GAME_BASE.write());
    dbg!(&*SAVE_GAME_BASE.read());
}

fn normalize_slashes(path: &mut String) {
    *path = fs::canonicalize(&path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}

fn setup_console_masks() {
    let cena = "ICWEFDA";
    if let Some(mut p) = args::check_parm("-cout") {
        lprint!(OutputLevel::DEBUG, "mask for stdout console output: ");
        p += 1;
        if p != ARGS.read().len() && !ARGS.read()[p].starts_with('-') {
            *misc::lprint::OUTPUT_MASK.write() = OutputLevel::NONE;
            for c in ARGS.read()[p].chars() {
                let c = c.to_ascii_uppercase();
                if cena.contains(c) {
                    let pos = cena.find(c).unwrap();
                    *misc::lprint::OUTPUT_MASK.write() |= OutputLevel::from_bits(1 << pos).unwrap();
                    lprint!(OutputLevel::DEBUG, "{}", c);
                }
            }
        }
        lprint!(OutputLevel::DEBUG, "\n");
    }
    if let Some(mut p) = args::check_parm("-cerr") {
        lprint!(OutputLevel::DEBUG, "mask for stderr console output: ");
        p += 1;
        if p != ARGS.read().len() && !ARGS.read()[p].starts_with('-') {
            *misc::lprint::ERROR_MASK.write() = OutputLevel::NONE;
            for c in ARGS.read()[p].chars() {
                let c = c.to_ascii_uppercase();
                if cena.contains(c) {
                    let pos = cena.find(c).unwrap();
                    *misc::lprint::ERROR_MASK.write() |= OutputLevel::from_bits(1 << pos).unwrap();
                    lprint!(OutputLevel::DEBUG, "{}", c);
                }
            }
        }
        lprint!(OutputLevel::DEBUG, "\n");
    }
}

fn find_response_file() {
    for (i, arg) in ARGS.read().iter().enumerate() {
        if arg.starts_with('@') {
            let mut fname = format!("{}.rsp", &arg[1..]);
            let file_contents = misc::read_file(&fname);
            let file_contents = match file_contents {
                Err(_) => {
                    fname = format!("{}/{}.rsp", doom_exe_dir(), &arg[1..]);
                    misc::read_file(&fname)
                        .unwrap_or_else(|_| error(format!("No such response file: {}", fname)))
                }
                Ok(file_contents) => file_contents,
            };
            lprint!(OutputLevel::CONFIRM, "Found response file {}\n", fname);
            if file_contents.is_empty() {
                lprint!(OutputLevel::ERROR, "\nResponse file empty!\n");

                ARGS.write().remove(i);
                return;
            }

            let mut moreargs = Vec::from(&ARGS.read()[i + 1..]);
            let mut newargv = vec![ARGS.read()[0].clone()];

            let mut indexinfile = 1;
            let mut size = file_contents.len();
            loop {
                while size > 0 && file_contents[indexinfile].is_ascii_whitespace() {
                    indexinfile += 1;
                    size -= 1;
                }
                if size > 0 {
                    let mut quoted = false;
                    let mut s = Vec::<u8>::with_capacity(size + 1);

                    while size > 0 {
                        if !quoted && file_contents[indexinfile].is_ascii_whitespace() {
                            break;
                        }
                        if file_contents[indexinfile] == b'"' {
                            indexinfile += 1;
                            quoted = !quoted;
                        } else {
                            s.push(file_contents[indexinfile]);
                        }
                        size -= 1;
                    }
                    if quoted {
                        error("Runaway quoted string in response file");
                    }
                    newargv.push(String::from_utf8(s).unwrap());
                    indexinfile += 1;
                } else {
                    break;
                }
            }

            newargv.append(&mut moreargs);

            *ARGS.write() = newargv;

            lprint!(
                OutputLevel::CONFIRM,
                "{} command-line args:\n",
                ARGS.read().len()
            );
            for arg in &*ARGS.read() {
                lprint!(OutputLevel::CONFIRM, "{}\n", arg);
            }
            break;
        }
    }
}

const PRBOOM_DIR: &str = "/.prboom-plus";
lazy_static! {
    static ref DOOM_EXE_DIR: RwLock<Option<String>> = RwLock::new(None);
}
fn doom_exe_dir() -> String {
    if DOOM_EXE_DIR.read().is_none() {
        let home = std::env::var("HOME").unwrap();
        *DOOM_EXE_DIR.write() = Some(format!("{}/{}", home, PRBOOM_DIR));
        match std::fs::create_dir(&DOOM_EXE_DIR.read().as_ref().unwrap()) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {}
                _ => panic!("creating file: {}", e),
            },
        }
        normalize_slashes(&mut DOOM_EXE_DIR.write().as_mut().unwrap());
    }
    DOOM_EXE_DIR.read().as_ref().unwrap().clone()
}

fn doom_loop() {}
