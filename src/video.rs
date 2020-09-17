use crate::{
    configuration::VideoMode,
    configuration::{Configuration, ScreenResolution},
    lprint,
    misc::args::ArgList,
    OutputLevel,
};
use lazy_static::lazy_static;
use parking_lot::RwLock;

lazy_static! {
    pub static ref COPY_RECT: RwLock<fn()> = RwLock::new(|| ());
    pub static ref FILL_RECT: RwLock<fn()> = RwLock::new(|| ());
    pub static ref DRAW_NUM_PATCH: RwLock<fn()> = RwLock::new(|| ());
}

pub fn init() {
    // TODO determine if need to do anything here
    // original C source is below
    /*
    int  i;

    for (i = 0; i<NUM_SCREENS; i++) {
      screens[i].data = NULL;
      screens[i].not_on_heap = false;
      screens[i].width = 0;
      screens[i].height = 0;
      screens[i].byte_pitch = 0;
      screens[i].short_pitch = 0;
      screens[i].int_pitch = 0;
    }
    */
}

pub fn init_screen_resolution(
    sdl_video: &Option<sdl2::VideoSubsystem>,
    configuration: &mut Configuration,
) {
    let mut desired_screen_resolution = configuration.defaults.screen_resolution.clone();
    let mut w = desired_screen_resolution.width;
    let mut h = desired_screen_resolution.height;
    if configuration.sdl_window.is_none() {
        fill_screen_resolutions_list(sdl_video, configuration, &desired_screen_resolution);

        if let Some(p) = configuration.args.check_parm("-width") {
            if p < configuration.args.len() - 1 {
                desired_screen_resolution.width =
                    configuration.args[p + 1].parse().unwrap_or_else(|_| {
                        crate::error(format!(
                            "-width: bad parameter {}",
                            configuration.args[p + 1]
                        ))
                    });
            }
        }
        if let Some(p) = configuration.args.check_parm("-height") {
            if p < configuration.args.len() - 1 {
                desired_screen_resolution.height =
                    configuration.args[p + 1].parse().unwrap_or_else(|_| {
                        crate::error(format!(
                            "-height: bad parameter {}",
                            configuration.args[p + 1]
                        ))
                    });
            }
        }
        if configuration.args.check_parm("-fullscreen").is_some() {
            configuration.use_fullscreen = true;
        }
        if configuration.args.check_parm("-nofullscreen").is_some() {
            configuration.use_fullscreen = false;
        }
        configuration.desired_fullscreen = configuration.use_fullscreen;

        if configuration.args.check_parm("-window").is_some() {
            configuration.desired_fullscreen = true;
        }
        if configuration.args.check_parm("-nowindow").is_some() {
            configuration.desired_fullscreen = false;
        }

        if let Some(p) = configuration.args.check_parms(&["-geom", "-geometry"]) {
            if p + 1 < configuration.args.len() {
                let geom_regex = regex::Regex::new(r"(\d+)[xX](\d+)(.)?").unwrap();
                if geom_regex.is_match(&configuration.args[p + 1]) {
                    let geom = geom_regex.captures(&configuration.args[p + 1]).unwrap();
                    w = geom.get(1).unwrap().as_str().parse().unwrap();
                    h = geom.get(2).unwrap().as_str().parse().unwrap();

                    if let Some(c) = geom.get(3) {
                        if c.as_str() == "w" {
                            configuration.desired_fullscreen = false;
                        } else if c.as_str() == "f" {
                            configuration.desired_fullscreen = true;
                        }
                    }
                }
            }
        }
    }

    let mut mode = configuration.defaults.videomode;
    if let Some(i) = configuration.args.check_parm("-vidmode") {
        if i < configuration.args.len() - 1 {
            mode = get_mode_from_string(&configuration.args[i + 1]);
        }
    }

    init_mode(mode);
}

pub fn init_mode(mode: VideoMode) {
    match mode {
        VideoMode::Mode8 => {
            lprint!(
                OutputLevel::INFO,
                "video::init_mode: using 8 bit video mode\n"
            );
        }
        VideoMode::Mode15 => {}
        VideoMode::Mode16 => {}
        VideoMode::Mode32 => {}
        VideoMode::ModeGL => {}
    }
}

pub fn get_mode_from_string(mode_str: &str) -> VideoMode {
    match mode_str.to_lowercase().as_str() {
        "15" | "15bit" => VideoMode::Mode15,
        "16" | "16bit" => VideoMode::Mode16,
        "32" | "32bit" => VideoMode::Mode32,
        "gl" | "opengl" => VideoMode::ModeGL,
        _ => VideoMode::Mode8,
    }
}

pub fn fill_screen_resolutions_list(
    sdl_video: &Option<sdl2::VideoSubsystem>,
    configuration: &mut Configuration,
    desired_screen_resolution: &ScreenResolution,
) {
    if !configuration.screen_resolutions_list.is_empty() {
        return;
    }

    let display_index = 0;

    let count = if !configuration.no_draw {
        sdl_video
            .as_ref()
            .unwrap()
            .num_display_modes(display_index)
            .unwrap_or_else(|e| {
                crate::error(format!("Could not get number of display modes [{}]", e))
            })
    } else {
        0
    };

    let mut current_resolution_index: i32 = -1;
    if count > 0 {
        'list_check: for i in (0..count).rev() {
            let mode = sdl_video
                .as_ref()
                .unwrap()
                .display_mode(display_index, i)
                .unwrap_or_else(|e| {
                    crate::error(format!(
                        "Could not get display mode for display {} [{}]",
                        display_index, e
                    ))
                });
            let mode_name = format!("{}x{}", mode.w, mode.h);
            if i == count - 1 {
                configuration.screen_resolution_lowest = mode_name.clone();
            }

            for res in &configuration.screen_resolutions_list {
                if &mode_name == res {
                    break 'list_check;
                }
            }

            configuration.screen_resolutions_list.push(mode_name);
            if mode.w == desired_screen_resolution.width as i32
                && mode.h == desired_screen_resolution.height as i32
            {
                current_resolution_index = (configuration.screen_resolutions_list.len() - 1) as i32;
            }
        }
    }

    if configuration.screen_resolutions_list.is_empty() {
        let mode_name = format!(
            "{}x{}",
            desired_screen_resolution.width, desired_screen_resolution.height
        );
        configuration.screen_resolutions_list.push(mode_name);
        current_resolution_index = 0;
    }

    if current_resolution_index == -1 {
        let mode_name = format!(
            "{}x{}",
            desired_screen_resolution.width, desired_screen_resolution.height
        );
        configuration.screen_resolutions_list.insert(0, mode_name);
        current_resolution_index = 0;
    }

    configuration.screen_resolution = current_resolution_index as usize;
}
