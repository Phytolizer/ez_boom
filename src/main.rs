mod configuration;
use args::ArgList;
use configuration::{Configuration, SkillLevel};

mod defs;
mod deh;
mod doom;
mod game;
use game::{FORWARD_MOVE, SIDE_MOVE};
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

use defs::{PACKAGE_NAME, VERSION_DATE};
use doom::def::{GameMission, GameMode, Language};
use doom::english::DEVSTR;
use io::SeekFrom;
use std::{convert::TryFrom, env, fs, io, path::PathBuf};
use wad::{add_default_extension, FileLump, ReadWadExt, WadFileInfo, WadSource};

#[cfg(windows)]
const PATH_SEPARATOR: char = ';';
#[cfg(not(windows))]
const PATH_SEPARATOR: char = ':';

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

pub(crate) fn error<S: AsRef<str>>(why: S) -> ! {
    lprint!(OutputLevel::ERROR, "{}\n", why.as_ref());
    std::process::exit(-1);
}

fn read_configuration() -> Box<Configuration> {
    let mut configuration = Box::<Configuration>::default();

    // TODO

    configuration
}

fn read_args(configuration: &mut Configuration) {
    configuration.args.check_arg_conflicts();
}

fn print_version() {
    lprint!(OutputLevel::INFO, "{}\n", system::version_string());
}

fn main() {
    let mut configuration = read_configuration();
    read_args(&mut configuration);
    print_version();

    let sdl = pre_init_graphics();

    doom_main(&mut configuration);
}

fn pre_init_graphics() -> sdl2::Sdl {
    match sdl2::init() {
        Ok(sdl) => sdl,
        Err(e) => {
            error(format!("Could not initialize SDL [{}]", e));
        }
    }
}

fn doom_main(configuration: &mut Configuration) {
    doom_main_setup(configuration);

    doom_loop();
}

fn doom_main_setup(configuration: &mut Configuration) {
    setup_console_masks(configuration);

    loop {
        let mut rsp_found = false;
        for arg in &configuration.args {
            if arg.starts_with('@') {
                rsp_found = true;
            }
        }
        find_response_file(configuration);
        if !rsp_found {
            break;
        }
    }

    if configuration.args.check_parm("-forceoldbsp").is_some() {
        configuration.force_old_bsp = true;
    }

    deh::build_bex_tables();

    configuration.args.handle_loose_files();

    identify_version(configuration);

    configuration.arg_meta.nomonsters = configuration.args.check_parm("-nomonsters").is_some();
    configuration.nomonsters = configuration.arg_meta.nomonsters;
    configuration.arg_meta.respawnparm = configuration.args.check_parm("-respawn").is_some();
    configuration.respawnparm = configuration.arg_meta.respawnparm;
    configuration.arg_meta.fastparm = configuration.args.check_parm("-fast").is_some();
    configuration.fastparm = configuration.arg_meta.fastparm;

    configuration.devparm = configuration.args.check_parm("-devparm").is_some();
    configuration.deathmatch = if configuration.args.check_parm("-altdeath").is_some() {
        2
    } else if configuration.args.check_parm("-deathmatch").is_some() {
        1
    } else {
        0
    };

    configuration.doom_ver_str = String::from(match configuration.game_mode {
        GameMode::Retail => match configuration.game_mission {
            GameMission::Chex => "Chex(R) Quest",
            _ => "The Ultimate DOOM",
        },
        GameMode::Shareware => "DOOM Shareware",
        GameMode::Registered => "DOOM Registered",
        GameMode::Commercial => match configuration.game_mission {
            GameMission::Plutonia => "Final DOOM - The Plutonia Experiment",
            GameMission::TNT => "Final DOOM - TNT: Evilution",
            GameMission::Hacx => "HACX - Twitch 'n Kill",
            _ => "DOOM 2: Hell on Earth",
        },
        _ => "Public DOOM",
    });

    if configuration.bfg_edition {
        configuration.doom_ver_str.push_str(" (BFG Edition)");
    }

    lprint!(
        OutputLevel::ALWAYS,
        "{0} (built {1}), playing {2}\n\
    {0} is released under the GNU General Public license v2.0.\n\
    You are welcome to redistribute it under certain conditions.\n\
    It comes with ABSOLUTELY NO WARRANTY.\n",
        PACKAGE_NAME,
        VERSION_DATE,
        configuration.doom_ver_str
    );

    if configuration.devparm {
        lprint!(OutputLevel::CONFIRM, "{}", DEVSTR);
    }

    let p = match configuration.args.check_parm("-turbo") {
        Some(it) => it,
        _ => return,
    };
    let scale = if p < configuration.args.len() - 1 {
        configuration.args[p + 1].parse::<i32>().unwrap_or(0)
    } else {
        200
    };
    let scale = num::clamp(scale, 10, 400);

    lprint!(OutputLevel::CONFIRM, "turbo scale: {}%\n", scale);
    {
        let mut forward_move = FORWARD_MOVE.write();
        *forward_move = [forward_move[0] * scale / 100, forward_move[1] * scale / 100];
    }
    {
        let mut side_move = SIDE_MOVE.write();
        *side_move = [side_move[0] * scale / 100, side_move[1] * scale / 100];
    }

    if let Some(p) = configuration.args.check_parm("-skill") {
        if p < configuration.args.len() - 1 {
            configuration.start_skill =
                SkillLevel::try_from(configuration.args[p + 1].as_bytes()[0] - b'1')
                    .unwrap_or_else(|e| error(e));
        }
    }
}

fn identify_version(configuration: &mut Configuration) {
    configuration.save_game_base =
        PathBuf::from(env::var("DOOMSAVEDIR").unwrap_or_else(|_| doom_exe_dir()));
    if let Some(i) = configuration.args.check_parm("-save") {
        if i < configuration.args.len() - 1 {
            let path = &configuration.args[i + 1];
            if let Ok(true) = fs::metadata(path).map(|m| m.is_dir()) {
                configuration.save_game_base = PathBuf::from(path);
            } else {
                lprint!(
                    OutputLevel::ERROR,
                    "Error: -save path does not exist. Using {} instead\n",
                    configuration.save_game_base.to_str().unwrap()
                );
            }
        }
    }
    let iwad = find_iwad_file(configuration);

    if let Some(iwad) = iwad {
        add_iwad(configuration, &iwad);
    } else {
        error("identify_version: IWAD not found\n");
    }
}

fn add_iwad(configuration: &mut Configuration, iwad: &str) {
    lprint!(OutputLevel::CONFIRM, "IWAD found: {}\n", iwad);
    check_iwad(configuration, iwad);

    match configuration.game_mode {
        GameMode::Retail | GameMode::Registered | GameMode::Shareware => {
            configuration.game_mission = if iwad.ends_with("chex.wad") {
                GameMission::Chex
            } else {
                GameMission::Doom
            };
        }
        GameMode::Commercial => {
            if iwad.ends_with("doom2f.wad") {
                configuration.language = Language::French;
            }

            configuration.game_mission = if iwad.ends_with("tnt.wad") {
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
    add_file(configuration, iwad, WadSource::Iwad);
}

fn add_file(configuration: &mut Configuration, file: &str, source: WadSource) {
    configuration.wad_files.push(WadFileInfo {
        name: PathBuf::from(add_default_extension(file, ".wad")),
        src: source,
        handle: 0,
    });

    let info = configuration.wad_files.iter().last().unwrap();
    if info.name.ends_with("nerve.wad") {
        configuration.game_mission = GameMission::Nerve;
    }

    let gwa_filename = add_default_extension(file, ".wad");
    if gwa_filename.ends_with(".wad") {
        let mut gwa_filename = gwa_filename.into_bytes();
        let n = gwa_filename.len();
        gwa_filename[n - 3] = b'g';
        gwa_filename[n - 2] = b'w';
        gwa_filename[n - 1] = b'a';
        let gwa_filename = String::from_utf8(gwa_filename).unwrap();

        configuration.wad_files.push(WadFileInfo {
            name: PathBuf::from(gwa_filename),
            src: source,
            handle: 0,
        });
    }
}

fn check_iwad(configuration: &mut Configuration, iwad: &str) {
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
    let mut ultdoom_levels = 0;
    // Doom Registered
    let mut registered_levels = 0;
    // Doom Shareware
    let mut shareware_levels = 0;
    // Doom II ("commmercial")
    let mut commercial_levels = 0;
    // Secret level
    let mut secret_levels = 0;
    // Hacx
    let mut hacx_levels = 0;
    // ?
    let mut cq = 0;
    while length > 0 {
        length -= 1;
        let lump = &fileinfo[length as usize];
        if lump.name[0] == b'E' && lump.name[2] == b'M' && lump.name[4] == b'\0' {
            // ExMy
            match lump.name[1] {
                b'4' => ultdoom_levels += 1,
                b'3' => registered_levels += 1,
                b'2' => registered_levels += 1,
                b'1' => shareware_levels += 1,
                _ => {}
            }
        } else if &lump.name[0..3] == b"MAP" && lump.name[5] == b'\0' {
            commercial_levels += 1;
            let level_num = &lump.name[3..5];
            if level_num == b"31" || level_num == b"32" {
                secret_levels += 1;
            }
        }
        if &lump.name[0..8] == b"DMENUPIC" {
            configuration.bfg_edition = true;
        }
        if &lump.name[0..4] == b"HACX" {
            hacx_levels += 1;
        }
        if &lump.name[0..5] == b"W94_1" || &lump.name[0..8] == b"POSSHOM0" {
            cq += 1;
        }
    }

    if noiwad && !configuration.bfg_edition && cq < 2 {
        error(format!("check_iwad: IWAD tag not present for {}", iwad));
    }

    configuration.game_mode =
        if commercial_levels >= 30 || (commercial_levels >= 20 && hacx_levels > 0) {
            configuration.has_wolf_levels = secret_levels >= 2;
            GameMode::Commercial
        } else if ultdoom_levels >= 9 {
            GameMode::Retail
        } else if registered_levels >= 18 {
            GameMode::Registered
        } else if shareware_levels >= 9 {
            GameMode::Shareware
        } else {
            GameMode::TBD
        };
}

fn find_iwad_file(configuration: &Configuration) -> Option<String> {
    if let Some(mut i) = configuration.args.check_parm("-iwad") {
        i += 1;
        if i < configuration.args.len() {
            return find_file(&configuration.args[i], ".wad");
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
        let d = if let Some(ref var) = path.env_var {
            match env::var(var) {
                Ok(v) => Some(v),
                Err(_) => continue,
            }
        } else if let Some(func) = path.func {
            Some(func())
        } else if let Some(ref abs) = path.absolute_dir {
            Some(abs.clone())
        } else {
            None
        };

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

fn setup_console_masks(configuration: &Configuration) {
    let cena = "ICWEFDA";
    if let Some(mut p) = configuration.args.check_parm("-cout") {
        lprint!(OutputLevel::DEBUG, "mask for stdout console output: ");
        p += 1;
        if p != configuration.args.len() && !configuration.args[p].starts_with('-') {
            *misc::lprint::OUTPUT_MASK.write() = OutputLevel::NONE;
            for c in configuration.args[p].chars() {
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
    if let Some(mut p) = configuration.args.check_parm("-cerr") {
        lprint!(OutputLevel::DEBUG, "mask for stderr console output: ");
        p += 1;
        if p != configuration.args.len() && !configuration.args[p].starts_with('-') {
            *misc::lprint::ERROR_MASK.write() = OutputLevel::NONE;
            for c in configuration.args[p].chars() {
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

fn find_response_file(configuration: &mut Configuration) {
    for (i, arg) in configuration.args.iter().enumerate() {
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

                configuration.args.remove(i);
                return;
            }

            let mut moreargs = Vec::from(&configuration.args[i + 1..]);
            let mut newargv = vec![configuration.args[0].clone()];

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

            configuration.args = newargv;

            lprint!(
                OutputLevel::CONFIRM,
                "{} command-line args:\n",
                configuration.args.len()
            );
            for arg in &configuration.args {
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
