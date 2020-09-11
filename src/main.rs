mod configuration;
use configuration::Configuration;

mod defs;
mod deh;
mod doom;
mod game;
mod info;
mod logic;

mod misc;
use misc::args;
use misc::lprint::OutputLevel;

mod sounds;
mod system;
mod think;
mod wad;

use counted_array::counted_array;
use faccess::PathExt;
use lazy_static::lazy_static;
use parking_lot::RwLock;

use doom::def::{GameMission, GameMode, Language};
use doom::stat::{GAMEMISSION, GAMEMODE, LANGUAGE};
use game::HAS_WOLF_LEVELS;
use io::SeekFrom;
use std::{env, fs, io, mem, path::PathBuf, slice};
use wad::{add_default_extension, FileLump, ReadWadExt, WadFileInfo, WadSource, WADFILES};

#[cfg(windows)]
const PATH_SEPARATOR: char = ';';
#[cfg(not(windows))]
const PATH_SEPARATOR: char = ':';

lazy_static! {
    static ref ARGS: RwLock<Vec<String>> = RwLock::new(vec![]);
    pub static ref FORCE_OLD_BSP: RwLock<bool> = RwLock::new(false);
    static ref SAVE_GAME_BASE: RwLock<String> = RwLock::new(String::new());
}

counted_array!(
    const STANDARD_IWADS: [&str; _] = [
        "doom2f.wad",
        "doom2.wad",
        "plutonia.wad",
        "tnt.wad",
        "doom.wad",
        "doom1.wad",
        "doomu.wad",
        "freedoom2.wad",
        "freedoom1.wad",
        "freedm.wad",
        "hacx.wad",
        "chex.wad",
        "bfgdoom2.wad",
        "bfgdoom.wad",
    ]
);

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

    let iwad = find_iwad_file();

    if let Some(iwad) = iwad {
        add_iwad(&iwad);
    } else {
        error("identify_version: IWAD not found\n");
    }
}

fn add_iwad(iwad: &str) {
    lprint!(OutputLevel::CONFIRM, "IWAD found: {}\n", iwad);
    check_iwad(iwad);

    match *GAMEMODE.read() {
        GameMode::Retail | GameMode::Registered | GameMode::Shareware => {
            *GAMEMISSION.write() = if iwad.ends_with("chex.wad") {
                GameMission::Chex
            } else {
                GameMission::Doom
            };
        }
        GameMode::Commercial => {
            if iwad.ends_with("doom2f.wad") {
                *LANGUAGE.write() = Language::French;
            }

            *GAMEMISSION.write() = if iwad.ends_with("tnt.wad") {
                GameMission::TNT
            } else if iwad.ends_with("plutonia.wad") {
                GameMission::Plutonia
            } else if iwad.ends_with("hacx.wad") {
                GameMission::Hacx
            } else {
                GameMission::Doom2
            };
        }
        GameMode::TBD => {
            lprint!(OutputLevel::WARN, "Unknown game version, may not work\n");
        }
    }
    add_file(iwad, WadSource::Iwad);
}

fn add_file(file: &str, source: WadSource) {
    WADFILES.write().push(WadFileInfo {
        name: add_default_extension(file, ".wad"),
        src: source,
        handle: 0,
    });

    let wadfiles = WADFILES.read();
    let info = wadfiles.iter().last().unwrap();
    if info.name.ends_with("nerve.wad") {
        *GAMEMISSION.write() = GameMission::Nerve;
    }

    let gwa_filename = add_default_extension(file, ".wad");
    if gwa_filename.ends_with(".wad") {
        let mut gwa_filename = gwa_filename.into_bytes();
        let n = gwa_filename.len();
        gwa_filename[n - 3] = b'g';
        gwa_filename[n - 2] = b'w';
        gwa_filename[n - 1] = b'a';
        let gwa_filename = String::from_utf8(gwa_filename).unwrap();

        WADFILES.write().push(WadFileInfo {
            name: gwa_filename,
            src: source,
            handle: 0,
        });
    }
}

fn check_iwad(iwad: &str) {
    use std::io::Seek;

    if PathBuf::from(iwad)
        .access(faccess::AccessMode::READ)
        .is_err()
    {
        error(format!("check_iwad: IWAD {} is not readable", iwad));
    }

    let f = fs::File::open(iwad);
    if f.is_err() {
        error(format!("check_iwad: Can't open IWAD {}", iwad));
    }
    let mut f = f.unwrap();
    let header = f.read_wadinfo().unwrap();

    let noiwad = &header.identification != b"IWAD";
    let mut fileinfo = Vec::<FileLump>::new();

    f.seek(SeekFrom::Start(header.infotableofs as u64))
        .and_then(|_| {
            for _ in 0..header.numlumps {
                fileinfo.push(f.read_filelump()?);
            }
            Ok(())
        })
        .map(|_| {
            drop(f);
        })
        .unwrap_or_else(|_| {
            error(format!("check_iwad: failed to read directory {}", iwad));
        });

    let mut length = header.numlumps;
    // Ultimate Doom
    let mut ud = 0;
    // Doom Registered
    let mut rg = 0;
    // Doom Shareware
    let mut sw = 0;
    // Doom II ("commmercial")
    let mut cm = 0;
    // Secret level
    let mut sc = 0;
    // Hacx
    let mut hx = 0;
    // ?
    let mut cq = 0;
    let mut bfgedition = false;
    while length > 0 {
        length -= 1;
        let lump = &fileinfo[length as usize];
        if lump.name[0] == b'E' && lump.name[2] == b'M' && lump.name[4] == b'\0' {
            // ExMy
            match lump.name[1] {
                b'4' => ud += 1,
                b'3' => rg += 1,
                b'2' => rg += 1,
                b'1' => sw += 1,
                _ => {}
            }
        } else if &lump.name[0..3] == b"MAP" && lump.name[5] == b'\0' {
            cm += 1;
            if lump.name[3] == b'3' && (lump.name[4] == b'1' || lump.name[4] == b'2') {
                sc += 1;
            }
        }
        if &lump.name[0..8] == b"DMENUPIC" {
            bfgedition = true;
        }
        if &lump.name[0..4] == b"HACX" {
            hx += 1;
        }
        if &lump.name[0..5] == b"W94_1" || &lump.name[0..8] == b"POSSHOM0" {
            cq += 1;
        }
    }

    if noiwad && !bfgedition && cq < 2 {
        error(format!("check_iwad: IWAD tag not present for {}", iwad));
    }

    *GAMEMODE.write() = GameMode::TBD;
    *HAS_WOLF_LEVELS.write() = false;
    if cm >= 30 || (cm >= 20 && hx > 0) {
        *GAMEMODE.write() = GameMode::Commercial;
        *HAS_WOLF_LEVELS.write() = sc >= 2;
    } else if ud >= 9 {
        *GAMEMODE.write() = GameMode::Retail;
    } else if rg >= 18 {
        *GAMEMODE.write() = GameMode::Registered;
    } else if sw >= 9 {
        *GAMEMODE.write() = GameMode::Shareware;
    }
}

fn find_iwad_file() -> Option<String> {
    if let Some(mut i) = args::check_parm("-iwad") {
        i += 1;
        if i < ARGS.read().len() {
            return find_file(&ARGS.read()[i], ".wad");
        }
    }
    let mut iwad: Option<String> = None;
    let mut i = 0;
    while iwad.is_none() {
        if i == STANDARD_IWADS.len() {
            return None;
        }
        iwad = find_file(STANDARD_IWADS[i], ".wad");
        i += 1;
    }
    iwad
}

fn find_file(name: &str, ext: &str) -> Option<String> {
    find_file_internal(name, ext, false)
}

#[derive(Clone)]
struct SearchPath {
    absolute_dir: Option<String>,
    subdir: Option<String>,
    env_var: Option<String>,
    func: Option<fn() -> String>,
}

impl SearchPath {
    fn absolute(dir: &str) -> Self {
        Self {
            absolute_dir: Some(dir.to_string()),
            subdir: None,
            env_var: None,
            func: None,
        }
    }

    fn func(f: fn() -> String) -> Self {
        Self {
            absolute_dir: None,
            subdir: None,
            env_var: None,
            func: Some(f),
        }
    }

    fn env(var: &str) -> Self {
        Self {
            absolute_dir: None,
            subdir: None,
            env_var: Some(var.to_string()),
            func: None,
        }
    }

    fn env_and_subdir(var: &str, subdir: &str) -> Self {
        Self {
            absolute_dir: None,
            subdir: Some(subdir.to_string()),
            env_var: Some(var.to_string()),
            func: None,
        }
    }

    fn nothing() -> Self {
        Self {
            absolute_dir: None,
            subdir: None,
            env_var: None,
            func: None,
        }
    }
}

fn find_file_internal(name: &str, ext: &str, is_static: bool) -> Option<String> {
    lazy_static! {
        static ref NUM_SEARCH: RwLock<Option<usize>> = RwLock::new(None);
        static ref SEARCH: RwLock<Vec<SearchPath>> = RwLock::new(vec![]);
        static ref STATIC_P: RwLock<String> = RwLock::new(String::new());
    }
    let search_static = [
        SearchPath::func(doom_exe_dir),
        SearchPath::nothing(),
        SearchPath::env("DOOMWADDIR"),
        SearchPath::absolute(env!("DOOMWADDIR")),
        SearchPath::env_and_subdir("HOME", "doom"),
        SearchPath::env_and_subdir("HOME", "doom/iwad"),
        SearchPath::env_and_subdir("HOME", "doom/pwad/all"),
        SearchPath::env("HOME"),
        SearchPath::absolute("/usr/local/share/games/doom"),
        SearchPath::absolute("/usr/share/games/doom"),
        SearchPath::absolute("/usr/local/share/doom"),
        SearchPath::absolute("/usr/share/doom"),
    ];

    if NUM_SEARCH.read().is_none() {
        // initialize it
        *NUM_SEARCH.write() = Some(search_static.len());
        *SEARCH.write() = search_static.to_vec();

        // add each dir from $DOOMWADPATH
        if let Ok(dwp) = env::var("DOOMWADPATH") {
            dwp.split(PATH_SEPARATOR)
                .map(|p| SearchPath::absolute(p))
                .for_each(|p| SEARCH.write().push(p));
        }
    }

    for path in &*SEARCH.read() {
        let path: &SearchPath = path;
        let mut d = Some(String::new());
        if let Some(ref var) = path.env_var {
            match env::var(var) {
                Ok(v) => d = Some(v),
                Err(_) => continue,
            }
        } else if let Some(func) = path.func {
            d = Some(func());
        } else if let Some(ref abs) = path.absolute_dir {
            d = Some(abs.clone());
        } else {
            d = None;
        }

        let s = path.subdir.clone();
        let dynamic_p = RwLock::new(String::new());
        let mut p = if is_static {
            STATIC_P.write()
        } else {
            dynamic_p.write()
        };
        *p = format!(
            "{}{}{}{}{}",
            if let Some(ref d) = d { d } else { "" },
            if let Some(true) = d.as_ref().map(|d| !has_trailing_slash(&d)) {
                "/"
            } else {
                ""
            },
            if let Some(ref s) = s { s } else { "" },
            if let Some(true) = s.as_ref().map(|s| !has_trailing_slash(&s)) {
                "/"
            } else {
                ""
            },
            name
        );
        if !PathBuf::from(&*p).exists() && !ext.is_empty() {
            p.push_str(ext);
        }
        if PathBuf::from(&*p).exists() {
            if !is_static {
                lprint!(OutputLevel::INFO, " found {}\n", &*p);
            }
            return Some(p.to_string());
        }
    }
    None
}

#[cfg(windows)]
fn has_trailing_slash(s: &str) -> bool {
    s.ends_with('\\')
}

#[cfg(not(windows))]
fn has_trailing_slash(s: &str) -> bool {
    s.ends_with('/')
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
