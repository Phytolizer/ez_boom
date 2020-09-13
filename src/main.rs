//! A Boom-derived sourceport for Doom.

/// Contains the monolithic Configuration struct, which holds most of the
/// values needed upon initializing and running the game.
mod configuration;

/// Contains some definitions needed for helpful output, such as the
/// package name and version.
mod defs;
/// Contains utilities related to .deh and .bex file reading/parsing.
mod deh;
/// Contains some core game logic, and useful definitions.
mod doom;
/// TODO: Contains nothing yet.
mod game;
/// Contains many enums related to game code, such as state identifiers.
mod info;
/// Contains some core game logic.
mod logic;

/// Contains miscellaneous functions and structs that just don't fit anywhere else.
mod misc;

/// Contains enums and structs related to music and sound effects.
mod sounds;
/// Contains useful system-level functions.
mod system;
/// Contains useful constants.
mod tables;
/// Contains types related to thinkers.
mod think;
/// Contains types and functions related to reading the .wad format.
mod wad;

use counted_array::counted_array;
use faccess::PathExt;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use regex::Regex;

use args::ArgList;
use configuration::{Configuration, SkillLevel};
use defs::{PACKAGE_NAME, VERSION_DATE};
use doom::def::{GameMission, GameMode, Language};
use doom::english::DEVSTR;
use io::SeekFrom;
use misc::args;
use misc::lprint::OutputLevel;
use std::{convert::TryFrom, env, fs, io, path::PathBuf};
use tables::ANG45;
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

/// Print an error and quit the program.
pub(crate) fn error<S: AsRef<str>>(why: S) -> ! {
    lprint!(OutputLevel::ERROR, "{}\n", why.as_ref());
    std::process::exit(-1);
}

/// Read the configuration file, if it is present.
fn read_configuration() -> Box<Configuration> {
    let mut configuration = Box::<Configuration>::default();

    misc::load_defaults(&mut configuration);

    configuration
}

/// Read command-line arguments and modify the configuration as necessary. This
/// function can fail if conflicting arguments were provided.
fn read_args(configuration: &mut Configuration) {
    configuration.args.check_arg_conflicts();
}

/// Print the version of the program.
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

/// Initialize the SDL library.
fn pre_init_graphics() -> sdl2::Sdl {
    match sdl2::init() {
        Ok(sdl) => sdl,
        Err(e) => {
            error(format!("Could not initialize SDL [{}]", e));
        }
    }
}

/// Run some final setup and enter the game loop.
fn doom_main(configuration: &mut Configuration) {
    doom_main_setup(configuration);

    doom_loop();
}

/// Setup that is required for Doom to run. Contains much argument
/// handling.
fn doom_main_setup(configuration: &mut Configuration) {
    setup_console_masks(configuration);

    loop {
        // Are there more response files to parse?
        let rsp_found = configuration.args.iter().any(|arg| arg.starts_with('@'));
        // This function call removes the first arg starting with @ so that the loop
        // can terminate.
        find_response_file(configuration);
        if !rsp_found {
            // All done, or there were no response files to begin with
            break;
        }
    }

    if configuration.args.check_parm("-forceoldbsp").is_some() {
        configuration.force_old_bsp = true;
    }

    deh::build_bex_tables();

    // make args well-formed by prefixing with -file/-deh/-playdemo
    configuration.args.handle_loose_files();

    // figure out what this IWAD thingy is
    identify_version(configuration);

    // lots of arg handling below, beware!
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

    // set this string, it'll be printed in a minute so it's important
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
        // this IWAD is very weird, so we'll just call it
        _ => "Public DOOM",
    });

    // append BFG edition to shame those who use those IWADs. They probably deserve it.
    if configuration.bfg_edition {
        configuration.doom_ver_str.push_str(" (BFG Edition)");
    }

    // print some information about our program! :)
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

    // epic developer message, lets you know you got in the mainframe
    if configuration.devparm {
        lprint!(OutputLevel::CONFIRM, "{}", DEVSTR);
    }

    // more nightmarish arg handling. dear god... so many parameters! and none of them
    // documented!
    if let Some(p) = configuration.args.check_parm("-turbo") {
        let scale = if p < configuration.args.len() - 1 {
            configuration.args[p + 1].parse::<i32>().unwrap_or(0)
        } else {
            200
        };
        let scale = num::clamp(scale, 10, 400);

        lprint!(OutputLevel::CONFIRM, "Turbo scale: {}%.\n", scale);
        configuration.forward_move = [
            configuration.forward_move[0] * scale / 100,
            configuration.forward_move[1] * scale / 100,
        ];
        configuration.side_move = [
            configuration.side_move[0] * scale / 100,
            configuration.side_move[1] * scale / 100,
        ];
    }

    if let Some(p) = configuration.args.check_parm("-skill") {
        if p < configuration.args.len() - 1 {
            configuration.start_skill =
                SkillLevel::try_from(configuration.args[p + 1].as_bytes()[0] - b'1')
                    .unwrap_or_else(|e| error(e));

            lprint!(
                OutputLevel::CONFIRM,
                "Skill level set to {}.",
                configuration.start_skill
            );
        }
    }

    if let Some(p) = configuration.args.check_parm("-episode") {
        if p < configuration.args.len() - 1 {
            configuration.start_episode =
                (configuration.args[p + 1].as_bytes()[0].saturating_sub(b'0')) as usize;
            lprint!(
                OutputLevel::CONFIRM,
                "Starting on episode {}.\n",
                configuration.start_episode
            );
        }
    }

    if let Some(p) = configuration.args.check_parm("-timer") {
        if p < configuration.args.len() - 1 && configuration.deathmatch > 0 {
            let time: usize = configuration.args[p + 1].parse().unwrap_or_else(|_| {
                error(format!(
                    "Error: non-integer timer: {}",
                    configuration.args[p + 1]
                ))
            });
            lprint!(
                OutputLevel::CONFIRM,
                "Levels will end after {} minute{}.\n",
                time,
                if time != 1 { "s" } else { "" }
            );

            // FIXME time is not actually used to set any variable here
        }
    }

    if let Some(p) = configuration.args.check_parm("-avg") {
        if p < configuration.args.len() - 1 && configuration.deathmatch > 0 {
            lprint!(
                OutputLevel::CONFIRM,
                "Austin Virtual Gaming: Levels will end after 20 minutes.\n"
            );
            // FIXME set a variable???
        }
    }

    if let Some(p) = configuration.args.check_parms(&["-warp", "-wart"]) {
        configuration.start_map = 1;
        configuration.autostart = true;
        if p < configuration.args.len() - 1 {
            let starts_with_number_regex = Regex::new(r"^(\d+)").unwrap();
            if configuration.game_mode == GameMode::Commercial {
                configuration.start_map =
                    match starts_with_number_regex.captures(&configuration.args[p + 1]) {
                        Some(cap) => cap.get(1).unwrap().as_str().parse().unwrap(),
                        None => 1,
                    };
                lprint!(
                    OutputLevel::CONFIRM,
                    "Warping to map {}.\n",
                    configuration.start_map
                );
            } else {
                if let Some(cap) = starts_with_number_regex.captures(&configuration.args[p + 1]) {
                    // unwrapping because it matched the regex already
                    configuration.start_episode = cap.get(1).unwrap().as_str().parse().unwrap();
                    configuration.start_map = 1;
                    if p < configuration.args.len() - 2 {
                        if let Some(cap) =
                            starts_with_number_regex.captures(&configuration.args[p + 2])
                        {
                            configuration.start_map = cap.get(1).unwrap().as_str().parse().unwrap();
                        }
                    }
                }
                lprint!(
                    OutputLevel::CONFIRM,
                    "Warping to episode {}, map {}.\n",
                    configuration.start_episode,
                    configuration.start_map
                );
            }
        }
    }

    let no_sound = configuration.args.check_parm("-nosound").is_some();
    configuration.no_music = no_sound || configuration.args.check_parm("-nomusic").is_some();
    configuration.no_sfx = no_sound || configuration.args.check_parm("-nosfx").is_some();

    configuration.no_draw = configuration.args.check_parm("-nodraw").is_some();
    configuration.no_blit = configuration.args.check_parm("-noblit").is_some();

    if let Some(p) = configuration.args.check_parm("-viewangle") {
        if p < configuration.args.len() - 1 {
            configuration.view_angle_offset = configuration.args[p + 1].parse().unwrap_or(0);
            configuration.view_angle_offset = num::clamp(configuration.view_angle_offset, 0, 7);
            configuration.view_angle_offset = (8 - configuration.view_angle_offset) * ANG45 as i32;
        }
    }

    game::reload_defaults(configuration);
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
fn doom_exe_dir() -> String {
    lazy_static! {
        static ref DOOM_EXE_DIR: RwLock<Option<String>> = RwLock::new(None);
    }
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
