use bounded_integer::bounded_integer;
use num_enum::TryFromPrimitive;
use serde_derive::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};
use std::{convert::TryFrom, env, fmt::Display, path::PathBuf};
use strum_macros::EnumString;

use crate::{
    doom::def::GameMission,
    doom::def::GameMode,
    doom::def::Key,
    doom::def::Language,
    doom_exe_dir, misc,
    misc::args::{ArgMeta, Args},
    wad::WadFileInfo,
};

pub struct Configuration {
    pub defaults: Box<Defaults>,

    pub args: Args,
    pub arg_meta: ArgMeta,
    pub nomonsters: bool,
    pub respawnparm: bool,
    pub fastparm: bool,
    pub devparm: bool,

    // can also be 2
    pub deathmatch: usize,
    pub force_old_bsp: bool,

    pub game_mode: GameMode,
    pub game_mission: GameMission,
    pub language: Language,

    pub doom_ver_str: String,
    pub bfg_edition: bool,
    pub has_wolf_levels: bool,

    pub save_game_base: PathBuf,
    pub start_skill: SkillLevel,
    pub start_episode: usize,
    pub start_map: usize,
    pub autostart: bool,

    pub wad_files: Vec<WadFileInfo>,

    pub forward_move: [i32; 2],
    pub side_move: [i32; 2],

    pub no_music: bool,
    pub no_sfx: bool,

    pub no_draw: bool,
    pub no_blit: bool,

    pub view_angle_offset: i32,

    pub default_file: PathBuf,
    pub netgame: bool,

    pub weapon_recoil: bool,
    pub player_bobbing: bool,
    pub variable_friction: bool,
    pub allow_pushers: bool,
    pub monsters_remember: bool,
    pub monster_infighting: MonsterInfightingLevel,
    pub monster_backing: bool,
    pub monster_avoid_hazards: bool,
    pub monster_friction: bool,
    pub dogs: PlayerHelpers,
    pub friend_distance: FriendDistance,
    pub help_friends: bool,
    pub monkeys: bool,

    pub demo_playback: bool,
    pub single_demo: bool,
    pub net_demo: bool,

    pub player_in_game: Vec<bool>,

    pub console_player: usize,

    pub compatibility_level: CompatibilityLevel,

    pub comp_zombie: bool,
    /// monsters used to telefrag only on MAP30, now they do it for spawners only
    pub comp_telefrag: bool,
    /// MBF encourages things to drop off of overhangs
    pub comp_dropoff: bool,
    /// original Doom archvile bugs, like ghosts
    pub comp_vile: bool,
    /// original Doom limits the number of lost souls that Pain Elementals spawn
    pub comp_pain: bool,
    /// original Doom lets skulls be spit through walls by Pain Elementals
    pub comp_skull: bool,
    /// original Doom duplicated blazing door sound
    pub comp_blazing: bool,
    /// MBF made door lighting changes more gradual
    pub comp_doorlight: bool,
    /// improvements to the game physics
    pub comp_model: bool,
    /// fixes to God mode
    pub comp_god: bool,
    /// MBF encourages things to drop off of overhangs
    pub comp_falloff: bool,
    /// fixes for moving floors bugs
    pub comp_floors: bool,
    pub comp_skymap: bool,
    /// MBF AI change
    pub comp_pursuit: bool,
    /// monsters stuck in doors fix
    pub comp_doorstuck: bool,
    /// MBF AI change, monsters try to stay on lifts
    pub comp_staylift: bool,
    /// TODO see p_floor.c
    pub comp_stairs: bool,
    pub comp_infcheat: bool,
    /// allow zero tags in wads
    pub comp_zerotags: bool,
    /// enables keygrab and noclipping mancubus fireballs
    pub comp_moveblock: bool,
    /// objects which aren't on the map at game start respawn at (0,0)
    pub comp_respawn: bool,
    /// TODO see s_sound.c
    pub comp_sound: bool,
    /// emulate pre-Ultimate BossDeath behavior
    pub comp_666: bool,
    /// enable lost souls bouncing
    pub comp_soul: bool,
    /// 2s mid textures don't animate
    pub comp_maskedanim: bool,
    /// use Doom's buggy "Ouch" face code
    pub comp_ouchface: bool,
    /// Max health in DEH applies only to potions
    pub comp_maxhealth: bool,
    /// No predefined translucency for some things
    pub comp_translucency: bool,

    pub demo_insurance: DemoInsurance,
    pub dog_jumping: bool,

    pub rngseed: u32,
    pub gametic: i32,

    pub sdl_window: Option<sdl2::video::Window>,
    pub screen_resolutions_list: Vec<String>,
    pub screen_resolution: usize,
    pub screen_resolution_lowest: String,
    pub use_fullscreen: bool,
    pub desired_fullscreen: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        let defaults = Box::<Defaults>::default();
        Configuration {
            defaults: Box::default(),

            args: env::args().collect(),
            arg_meta: ArgMeta::default(),
            nomonsters: false,
            respawnparm: false,
            fastparm: false,
            devparm: false,
            deathmatch: 0,
            force_old_bsp: false,

            game_mode: GameMode::TBD,
            game_mission: GameMission::None,
            language: Language::English,

            doom_ver_str: String::new(),
            bfg_edition: false,
            has_wolf_levels: false,

            save_game_base: PathBuf::new(),
            start_skill: SkillLevel::None,
            start_episode: 1,
            start_map: 1,
            autostart: false,

            wad_files: vec![],

            forward_move: [0x19, 0x32],
            side_move: [0x18, 0x28],

            no_music: false,
            no_sfx: false,

            no_draw: false,
            no_blit: false,

            view_angle_offset: 0,

            default_file: doom_exe_dir().join(misc::BOOM_CFG),
            netgame: false,

            weapon_recoil: defaults.weapon_recoil,
            player_bobbing: defaults.player_bobbing,
            variable_friction: defaults.variable_friction,
            allow_pushers: defaults.allow_pushers,
            monsters_remember: defaults.monsters_remember,
            monster_infighting: defaults.monster_infighting,
            monster_backing: defaults.monster_backing,
            monster_avoid_hazards: defaults.monster_avoid_hazards,
            monster_friction: defaults.monster_friction,
            dogs: defaults.player_helpers,
            friend_distance: defaults.friend_distance,
            help_friends: defaults.help_friends,
            monkeys: defaults.monkeys,

            demo_playback: false,
            single_demo: false,
            net_demo: false,

            player_in_game: vec![true],

            console_player: 0,

            compatibility_level: defaults.default_compatibility_level,

            comp_zombie: defaults.comp_zombie,
            comp_telefrag: defaults.comp_telefrag,
            comp_dropoff: defaults.comp_dropoff,
            comp_vile: defaults.comp_vile,
            comp_pain: defaults.comp_pain,
            comp_skull: defaults.comp_skull,
            comp_blazing: defaults.comp_blazing,
            comp_doorlight: defaults.comp_doorlight,
            comp_model: defaults.comp_model,
            comp_god: defaults.comp_god,
            comp_falloff: defaults.comp_falloff,
            comp_floors: defaults.comp_floors,
            comp_skymap: defaults.comp_skymap,
            comp_pursuit: defaults.comp_pursuit,
            comp_doorstuck: defaults.comp_doorstuck,
            comp_staylift: defaults.comp_staylift,
            comp_stairs: defaults.comp_stairs,
            comp_infcheat: defaults.comp_infcheat,
            comp_zerotags: defaults.comp_zerotags,
            comp_moveblock: defaults.comp_moveblock,
            comp_respawn: false,
            comp_sound: defaults.comp_sound,
            comp_666: defaults.comp_666,
            comp_soul: defaults.comp_soul,
            comp_maskedanim: defaults.comp_maskedanim,
            comp_ouchface: defaults.comp_ouchface,
            comp_maxhealth: defaults.comp_maxhealth,
            comp_translucency: defaults.comp_translucency,

            demo_insurance: defaults.demo_insurance,
            dog_jumping: defaults.dog_jumping,
            rngseed: 1993,
            gametic: 0,

            sdl_window: None,
            screen_resolutions_list: vec![],
            screen_resolution: 0,
            screen_resolution_lowest: String::new(),
            use_fullscreen: defaults.use_fullscreen,
            desired_fullscreen: defaults.use_fullscreen,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Defaults {
    #[serde(default = "default_process_priority")]
    pub process_priority: ProcessPriority,
    #[serde(default = "default_default_compatibility_level")]
    pub default_compatibility_level: CompatibilityLevel,
    #[serde(default = "default_realtic_clock_rate")]
    pub realtic_clock_rate: PositiveInt,
    #[serde(default = "default_menu_background")]
    pub menu_background: bool,
    #[serde(default = "default_body_queue_size")]
    pub body_queue_size: OptionalLimit,
    #[serde(default = "default_flashing_hom")]
    pub flashing_hom: bool,
    #[serde(default = "default_demo_insurance")]
    pub demo_insurance: DemoInsurance,
    #[serde(default = "default_endoom_mode")]
    pub endoom_mode: EndoomMode,
    #[serde(default = "default_level_precache")]
    pub level_precache: bool,
    #[serde(default = "default_demo_smoothturns")]
    pub demo_smoothturns: DemoSmoothTurns,
    #[serde(default = "default_boom_autoswitch")]
    pub boom_autoswitch: bool,
    #[serde(default = "default_wad_files")]
    pub wad_files: Vec<PathBuf>,
    #[serde(default = "default_deh_files")]
    pub deh_files: Vec<PathBuf>,
    #[serde(default = "default_default_skill")]
    pub default_skill: SkillLevel,
    #[serde(default = "default_weapon_recoil")]
    pub weapon_recoil: bool,
    #[serde(default = "default_doom_weapon_toggles")]
    pub doom_weapon_toggles: bool,
    #[serde(default = "default_player_bobbing")]
    pub player_bobbing: bool,
    #[serde(default = "default_weapon_attack_alignment")]
    pub weapon_attack_alignment: WeaponAttackAlignment,
    #[serde(default = "default_monsters_remember")]
    pub monsters_remember: bool,
    #[serde(default = "default_monster_infighting")]
    pub monster_infighting: MonsterInfightingLevel,
    #[serde(default = "default_monster_backing")]
    pub monster_backing: bool,
    #[serde(default = "default_monster_avoid_hazards")]
    pub monster_avoid_hazards: bool,
    #[serde(default = "default_monkeys")]
    pub monkeys: bool,
    #[serde(default = "default_monster_friction")]
    pub monster_friction: bool,
    #[serde(default = "default_help_friends")]
    pub help_friends: bool,
    #[serde(default = "default_allow_pushers")]
    pub allow_pushers: bool,
    #[serde(default = "default_variable_friction")]
    pub variable_friction: bool,
    #[serde(default = "default_player_helpers")]
    pub player_helpers: PlayerHelpers,
    #[serde(default = "default_friend_distance")]
    pub friend_distance: FriendDistance,
    #[serde(default = "default_dog_jumping")]
    pub dog_jumping: bool,
    #[serde(default = "default_sts_always_red")]
    pub sts_always_red: bool,
    #[serde(default = "default_sts_pct_always_gray")]
    pub sts_pct_always_gray: bool,
    #[serde(default = "default_sts_traditional_keys")]
    pub sts_traditional_keys: bool,
    #[serde(default = "default_sts_armorcolor_type")]
    pub sts_armorcolor_type: bool,
    #[serde(default = "default_show_messages")]
    pub show_messages: bool,
    #[serde(default = "default_autorun")]
    pub autorun: bool,
    #[serde(default = "default_deh_apply_cheats")]
    pub deh_apply_cheats: bool,
    #[serde(default = "default_comp_zombie")]
    pub comp_zombie: bool,
    #[serde(default = "default_comp_infcheat")]
    pub comp_infcheat: bool,
    #[serde(default = "default_comp_stairs")]
    pub comp_stairs: bool,
    #[serde(default = "default_comp_telefrag")]
    pub comp_telefrag: bool,
    #[serde(default = "default_comp_dropoff")]
    pub comp_dropoff: bool,
    #[serde(default = "default_comp_falloff")]
    pub comp_falloff: bool,
    #[serde(default = "default_comp_staylift")]
    pub comp_staylift: bool,
    #[serde(default = "default_comp_doorstuck")]
    pub comp_doorstuck: bool,
    #[serde(default = "default_comp_pursuit")]
    pub comp_pursuit: bool,
    #[serde(default = "default_comp_vile")]
    pub comp_vile: bool,
    #[serde(default = "default_comp_pain")]
    pub comp_pain: bool,
    #[serde(default = "default_comp_skull")]
    pub comp_skull: bool,
    #[serde(default = "default_comp_blazing")]
    pub comp_blazing: bool,
    #[serde(default = "default_comp_doorlight")]
    pub comp_doorlight: bool,
    #[serde(default = "default_comp_god")]
    pub comp_god: bool,
    #[serde(default = "default_comp_skymap")]
    pub comp_skymap: bool,
    #[serde(default = "default_comp_floors")]
    pub comp_floors: bool,
    #[serde(default = "default_comp_model")]
    pub comp_model: bool,
    #[serde(default = "default_comp_zerotags")]
    pub comp_zerotags: bool,
    #[serde(default = "default_comp_moveblock")]
    pub comp_moveblock: bool,
    #[serde(default = "default_comp_sound")]
    pub comp_sound: bool,
    #[serde(default = "default_comp_666")]
    pub comp_666: bool,
    #[serde(default = "default_comp_soul")]
    pub comp_soul: bool,
    #[serde(default = "default_comp_maskedanim")]
    pub comp_maskedanim: bool,
    #[serde(default = "default_comp_ouchface")]
    pub comp_ouchface: bool,
    #[serde(default = "default_comp_maxhealth")]
    pub comp_maxhealth: bool,
    #[serde(default = "default_comp_translucency")]
    pub comp_translucency: bool,
    #[serde(default = "default_snd_pcspeaker")]
    pub snd_pcspeaker: bool,
    #[serde(default = "default_sound_card")]
    pub sound_card: SoundCard,
    #[serde(default = "default_music_card")]
    pub music_card: MusicCard,
    #[serde(default = "default_pitched_sounds")]
    pub pitched_sounds: bool,
    #[serde(default = "default_samplerate")]
    pub samplerate: SampleRate,
    #[serde(default = "default_sfx_volume")]
    pub sfx_volume: Volume,
    #[serde(default = "default_music_volume")]
    pub music_volume: Volume,
    #[serde(default = "default_mus_pause_opt")]
    pub mus_pause_opt: MusicPauseOption,
    #[serde(default = "default_snd_channels")]
    pub snd_channels: SoundChannels,
    #[serde(default = "default_snd_midiplayer")]
    pub snd_midiplayer: MidiPlayer,
    #[serde(default = "default_snd_soundfont")]
    pub snd_soundfont: String,
    #[serde(default = "default_snd_mididev")]
    pub snd_mididev: String,
    #[serde(default = "default_mus_extend_volume")]
    pub mus_extend_volume: bool,
    #[serde(default = "default_mus_fluidsynth_chorus")]
    pub mus_fluidsynth_chorus: bool,
    #[serde(default = "default_mus_fluidsynth_reverb")]
    pub mus_fluidsynth_reverb: bool,
    #[serde(default = "default_mus_fluidsynth_gain")]
    pub mus_fluidsynth_gain: Gain,
    #[serde(default = "default_mus_opl_gain")]
    pub mus_opl_gain: Gain,
    #[serde(default = "default_videomode")]
    pub videomode: VideoMode,
    #[serde(default = "default_screen_resolution")]
    pub screen_resolution: ScreenResolution,
    #[serde(default = "default_use_fullscreen")]
    pub use_fullscreen: bool,
    #[serde(default = "default_render_vsync")]
    pub render_vsync: bool,
    #[serde(default = "default_translucency")]
    pub translucency: bool,
    #[serde(default = "default_tran_filter_pct")]
    pub tran_filter_pct: Percentage,
    #[serde(default = "default_screenblocks")]
    pub screenblocks: Screenblocks,
    #[serde(default = "default_usegamma")]
    pub usegamma: GammaCorrectionLevel,
    #[serde(default = "default_uncapped_framerate")]
    pub uncapped_framerate: bool,
    #[serde(default = "default_test_interpolation_method")]
    pub test_interpolation_method: InterpolationMethod,
    #[serde(default = "default_filter_wall")]
    pub filter_wall: Filter,
    #[serde(default = "default_filter_floor")]
    pub filter_floor: Filter,
    #[serde(default = "default_filter_sprite")]
    pub filter_sprite: Filter,
    #[serde(default = "default_filter_z")]
    pub filter_z: ZFilter,
    #[serde(default = "default_filter_patch")]
    pub filter_patch: Filter,
    #[serde(default = "default_filter_threshold")]
    pub filter_threshold: PositiveInt,
    #[serde(default = "default_sprite_edges")]
    pub sprite_edges: SlopedEdgeType,
    #[serde(default = "default_patch_edges")]
    pub patch_edges: SlopedEdgeType,
    #[serde(default = "default_gl_compatibility")]
    pub gl_compatibility: bool,
    #[serde(default = "default_gl_arb_multitexture")]
    pub gl_arb_multitexture: bool,
    #[serde(default = "default_gl_arb_texture_compression")]
    pub gl_arb_texture_compression: bool,
    #[serde(default = "default_gl_arb_texture_non_power_of_two")]
    pub gl_arb_texture_non_power_of_two: bool,
    #[serde(default = "default_gl_ext_arb_vertex_buffer_object")]
    pub gl_ext_arb_vertex_buffer_object: bool,
    #[serde(default = "default_gl_arb_pixel_buffer_object")]
    pub gl_arb_pixel_buffer_object: bool,
    #[serde(default = "default_gl_arb_shader_objects")]
    pub gl_arb_shader_objects: bool,
    #[serde(default = "default_gl_ext_blend_color")]
    pub gl_ext_blend_color: bool,
    #[serde(default = "default_gl_ext_framebuffer_object")]
    pub gl_ext_framebuffer_object: bool,
    #[serde(default = "default_gl_ext_packed_depth_stencil")]
    pub gl_ext_packed_depth_stencil: bool,
    #[serde(default = "default_gl_ext_texture_filter_anisotropic")]
    pub gl_ext_texture_filter_anisotropic: bool,
    #[serde(default = "default_gl_use_stencil")]
    pub gl_use_stencil: bool,
    #[serde(default = "default_gl_use_display_lists")]
    pub gl_use_display_lists: bool,
    #[serde(default = "default_gl_finish")]
    pub gl_finish: bool,
    #[serde(default = "default_gl_clear")]
    pub gl_clear: bool,
    #[serde(default = "default_gl_ztrick")]
    pub gl_ztrick: bool,
    #[serde(default = "default_gl_nearclip")]
    pub gl_nearclip: PositiveInt,
    #[serde(default = "default_gl_colorbuffer_bits")]
    pub gl_colorbuffer_bits: BufferBits,
    #[serde(default = "default_gl_depthbuffer_bits")]
    pub gl_depthbuffer_bits: BufferBits,
    #[serde(default = "default_gl_texture_filter")]
    pub gl_texture_filter: TextureFilter,
    #[serde(default = "default_gl_sprite_filter")]
    pub gl_sprite_filter: SpriteFilter,
    #[serde(default = "default_gl_patch_filter")]
    pub gl_patch_filter: PatchFilter,
    #[serde(default = "default_gl_texture_filter_anisotropic")]
    pub gl_texture_filter_anisotropic: AnisotropicFilter,
    #[serde(default = "default_gl_tex_format_string")]
    pub gl_tex_format_string: String,
    #[serde(default = "default_gl_sprite_offset")]
    pub gl_sprite_offset: SpriteOffset,
    #[serde(default = "default_gl_sprite_blend")]
    pub gl_sprite_blend: bool,
    #[serde(default = "default_gl_mask_sprite_threshold")]
    pub gl_mask_sprite_threshold: Percentage,
    #[serde(default = "default_gl_skymode")]
    pub gl_skymode: SkyType,
    #[serde(default = "default_gl_sky_detail")]
    pub gl_sky_detail: SkyDetail,
    #[serde(default = "default_gl_use_paletted_texture")]
    pub gl_use_paletted_texture: bool,
    #[serde(default = "default_gl_use_shared_texture_palette")]
    pub gl_use_shared_texture_palette: bool,
    #[serde(default = "default_use_mouse")]
    pub use_mouse: bool,
    #[serde(default = "default_mouse_sensitivity_horiz")]
    pub mouse_sensitivity_horiz: PositiveInt,
    #[serde(default = "default_mouse_sensitivity_vert")]
    pub mouse_sensitivity_vert: PositiveInt,
    #[serde(default = "default_mouseb_fire")]
    pub mouseb_fire: MouseButton,
    #[serde(default = "default_mouseb_strafe")]
    pub mouseb_strafe: MouseButton,
    #[serde(default = "default_mouseb_forward")]
    pub mouseb_forward: MouseButton,
    #[serde(default = "default_mouseb_backward")]
    pub mouseb_backward: MouseButton,
    #[serde(default = "default_mouseb_use")]
    pub mouseb_use: MouseButton,
    #[serde(default = "default_key_right")]
    pub key_right: Keycode,
    #[serde(default = "default_key_left")]
    pub key_left: Keycode,
    #[serde(default = "default_key_up")]
    pub key_up: Keycode,
    #[serde(default = "default_key_down")]
    pub key_down: Keycode,
    #[serde(default = "default_key_mlook")]
    pub key_mlook: Keycode,
    #[serde(default = "default_key_menu_right")]
    pub key_menu_right: Keycode,
    #[serde(default = "default_key_menu_left")]
    pub key_menu_left: Keycode,
    #[serde(default = "default_key_menu_up")]
    pub key_menu_up: Keycode,
    #[serde(default = "default_key_menu_down")]
    pub key_menu_down: Keycode,
    #[serde(default = "default_key_menu_backspace")]
    pub key_menu_backspace: Keycode,
    #[serde(default = "default_key_menu_escape")]
    pub key_menu_escape: Keycode,
    #[serde(default = "default_key_menu_enter")]
    pub key_menu_enter: Keycode,
    #[serde(default = "default_key_menu_clear")]
    pub key_menu_clear: Keycode,
    #[serde(default = "default_key_setup")]
    pub key_setup: Keycode,
    #[serde(default = "default_key_strafeleft")]
    pub key_strafeleft: Keycode,
    #[serde(default = "default_key_straferight")]
    pub key_straferight: Keycode,
    #[serde(default = "default_key_flyup")]
    pub key_flyup: Keycode,
    #[serde(default = "default_key_flydown")]
    pub key_flydown: Keycode,
    #[serde(default = "default_key_fire")]
    pub key_fire: Keycode,
    #[serde(default = "default_key_use")]
    pub key_use: Keycode,
    #[serde(default = "default_key_strafe")]
    pub key_strafe: Keycode,
    #[serde(default = "default_key_speed")]
    pub key_speed: Keycode,
    #[serde(default = "default_key_savegame")]
    pub key_savegame: Keycode,
    #[serde(default = "default_key_loadgame")]
    pub key_loadgame: Keycode,
    #[serde(default = "default_key_soundvolume")]
    pub key_soundvolume: Keycode,
    #[serde(default = "default_key_hud")]
    pub key_hud: Keycode,
    #[serde(default = "default_key_quicksave")]
    pub key_quicksave: Keycode,
    #[serde(default = "default_key_endgame")]
    pub key_endgame: Keycode,
    #[serde(default = "default_key_messages")]
    pub key_messages: Keycode,
    #[serde(default = "default_key_quickload")]
    pub key_quickload: Keycode,
    #[serde(default = "default_key_quit")]
    pub key_quit: Keycode,
    #[serde(default = "default_key_gamma")]
    pub key_gamma: Keycode,
    #[serde(default = "default_key_spy")]
    pub key_spy: Keycode,
    #[serde(default = "default_key_pause")]
    pub key_pause: Keycode,
    #[serde(default = "default_key_autorun")]
    pub key_autorun: Keycode,
    #[serde(default = "default_key_chat")]
    pub key_chat: Keycode,
    #[serde(default = "default_key_backspace")]
    pub key_backspace: Keycode,
    #[serde(default = "default_key_enter")]
    pub key_enter: Keycode,
    #[serde(default = "default_key_map")]
    pub key_map: Keycode,
    #[serde(default = "default_key_map_right")]
    pub key_map_right: Keycode,
    #[serde(default = "default_key_map_left")]
    pub key_map_left: Keycode,
    #[serde(default = "default_key_map_up")]
    pub key_map_up: Keycode,
    #[serde(default = "default_key_map_down")]
    pub key_map_down: Keycode,
    #[serde(default = "default_key_map_zoomin")]
    pub key_map_zoomin: Keycode,
    #[serde(default = "default_key_map_zoomout")]
    pub key_map_zoomout: Keycode,
    #[serde(default = "default_key_map_gobig")]
    pub key_map_gobig: Keycode,
    #[serde(default = "default_key_map_follow")]
    pub key_map_follow: Keycode,
    #[serde(default = "default_key_map_mark")]
    pub key_map_mark: Keycode,
    #[serde(default = "default_key_map_clear")]
    pub key_map_clear: Keycode,
    #[serde(default = "default_key_map_grid")]
    pub key_map_grid: Keycode,
    #[serde(default = "default_key_map_rotate")]
    pub key_map_rotate: Keycode,
    #[serde(default = "default_key_map_overlay")]
    pub key_map_overlay: Keycode,
    #[serde(default = "default_key_map_textured")]
    pub key_map_textured: Keycode,
    #[serde(default = "default_key_reverse")]
    pub key_reverse: Keycode,
    #[serde(default = "default_key_zoomin")]
    pub key_zoomin: Keycode,
    #[serde(default = "default_key_zoomout")]
    pub key_zoomout: Keycode,
    #[serde(default = "default_key_chatplayer1")]
    pub key_chatplayer1: Keycode,
    #[serde(default = "default_key_chatplayer2")]
    pub key_chatplayer2: Keycode,
    #[serde(default = "default_key_chatplayer3")]
    pub key_chatplayer3: Keycode,
    #[serde(default = "default_key_chatplayer4")]
    pub key_chatplayer4: Keycode,
    #[serde(default = "default_key_weapontoggle")]
    pub key_weapontoggle: Keycode,
    #[serde(default = "default_key_weapon1")]
    pub key_weapon1: Keycode,
    #[serde(default = "default_key_weapon2")]
    pub key_weapon2: Keycode,
    #[serde(default = "default_key_weapon3")]
    pub key_weapon3: Keycode,
    #[serde(default = "default_key_weapon4")]
    pub key_weapon4: Keycode,
    #[serde(default = "default_key_weapon5")]
    pub key_weapon5: Keycode,
    #[serde(default = "default_key_weapon6")]
    pub key_weapon6: Keycode,
    #[serde(default = "default_key_weapon7")]
    pub key_weapon7: Keycode,
    #[serde(default = "default_key_weapon8")]
    pub key_weapon8: Keycode,
    #[serde(default = "default_key_weapon9")]
    pub key_weapon9: Keycode,
    #[serde(default = "default_key_nextweapon")]
    pub key_nextweapon: Keycode,
    #[serde(default = "default_key_prevweapon")]
    pub key_prevweapon: Keycode,
    #[serde(default = "default_key_screenshot")]
    pub key_screenshot: Keycode,
    #[serde(default = "default_use_joystick")]
    pub use_joystick: bool,
    #[serde(default = "default_joy_left")]
    pub joy_left: usize,
    #[serde(default = "default_joy_right")]
    pub joy_right: usize,
    #[serde(default = "default_joy_up")]
    pub joy_up: usize,
    #[serde(default = "default_joy_down")]
    pub joy_down: usize,
    #[serde(default = "default_joyb_fire")]
    pub joyb_fire: usize,
    #[serde(default = "default_joyb_strafe")]
    pub joyb_strafe: usize,
    #[serde(default = "default_joyb_strafeleft")]
    pub joyb_strafeleft: usize,
    #[serde(default = "default_joyb_straferight")]
    pub joyb_straferight: usize,
    #[serde(default = "default_joyb_speed")]
    pub joyb_speed: usize,
    #[serde(default = "default_joyb_use")]
    pub joyb_use: usize,
    #[serde(default = "default_chatmacro0")]
    pub chatmacro0: String,
    #[serde(default = "default_chatmacro1")]
    pub chatmacro1: String,
    #[serde(default = "default_chatmacro2")]
    pub chatmacro2: String,
    #[serde(default = "default_chatmacro3")]
    pub chatmacro3: String,
    #[serde(default = "default_chatmacro4")]
    pub chatmacro4: String,
    #[serde(default = "default_chatmacro5")]
    pub chatmacro5: String,
    #[serde(default = "default_chatmacro6")]
    pub chatmacro6: String,
    #[serde(default = "default_chatmacro7")]
    pub chatmacro7: String,
    #[serde(default = "default_chatmacro8")]
    pub chatmacro8: String,
    #[serde(default = "default_chatmacro9")]
    pub chatmacro9: String,
    #[serde(default = "default_mapcolor_back")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_back: Color,
    #[serde(default = "default_mapcolor_grid")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_grid: Color,
    #[serde(default = "default_mapcolor_wall")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_wall: Color,
    #[serde(default = "default_mapcolor_fchg")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_fchg: Color,
    #[serde(default = "default_mapcolor_cchg")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_cchg: Color,
    #[serde(default = "default_mapcolor_clsd")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_clsd: Color,
    #[serde(default = "default_mapcolor_rkey")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_rkey: Color,
    #[serde(default = "default_mapcolor_bkey")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_bkey: Color,
    #[serde(default = "default_mapcolor_ykey")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_ykey: Color,
    #[serde(default = "default_mapcolor_rdor")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_rdor: Color,
    #[serde(default = "default_mapcolor_bdor")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_bdor: Color,
    #[serde(default = "default_mapcolor_ydor")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_ydor: Color,
    #[serde(default = "default_mapcolor_tele")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_tele: Color,
    #[serde(default = "default_mapcolor_secr")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_secr: Color,
    #[serde(default = "default_mapcolor_exit")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_exit: Color,
    #[serde(default = "default_mapcolor_unsn")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_unsn: Color,
    #[serde(default = "default_mapcolor_flat")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_flat: Color,
    #[serde(default = "default_mapcolor_sprt")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_sprt: Color,
    #[serde(default = "default_mapcolor_item")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_item: Color,
    #[serde(default = "default_mapcolor_hair")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_hair: Color,
    #[serde(default = "default_mapcolor_sngl")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_sngl: Color,
    #[serde(default = "default_mapcolor_me")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_me: Color,
    #[serde(default = "default_mapcolor_enemy")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_enemy: Color,
    #[serde(default = "default_mapcolor_frnd")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub mapcolor_frnd: Color,
    #[serde(default = "default_map_secret_after")]
    pub map_secret_after: bool,
    #[serde(default = "default_map_point_coord")]
    pub map_point_coord: bool,
    #[serde(default = "default_map_level_stat")]
    pub map_level_stat: bool,
    #[serde(default = "default_automapmode")]
    pub automapmode: AutomapMode,
    #[serde(default = "default_map_always_updates")]
    pub map_always_updates: bool,
    #[serde(default = "default_map_grid_size")]
    pub map_grid_size: MapGridSize,
    #[serde(default = "default_map_scroll_speed")]
    pub map_scroll_speed: MapScrollSpeed,
    #[serde(default = "default_map_wheel_zoom")]
    pub map_wheel_zoom: bool,
    #[serde(default = "default_map_use_multisampling")]
    pub map_use_multisampling: bool,
    #[serde(default = "default_map_textured")]
    pub map_textured: bool,
    #[serde(default = "default_map_textured_trans")]
    pub map_textured_trans: Percentage,
    #[serde(default = "default_map_textured_overlay_trans")]
    pub map_textured_overlay_trans: Percentage,
    #[serde(default = "default_map_lines_overlay_trans")]
    pub map_lines_overlay_trans: Percentage,
    #[serde(default = "default_map_overlay_pos_x")]
    pub map_overlay_pos_x: XPosition,
    #[serde(default = "default_map_overlay_pos_y")]
    pub map_overlay_pos_y: YPosition,
    #[serde(default = "default_map_overlay_pos_width")]
    pub map_overlay_pos_width: Width,
    #[serde(default = "default_map_overlay_pos_height")]
    pub map_overlay_pos_height: Height,
    #[serde(default = "default_map_things_appearance")]
    pub map_things_appearance: MapThingsAppearance,
    #[serde(default = "default_hudcolor_titl")]
    pub hudcolor_titl: TextColor,
    #[serde(default = "default_hudcolor_xyco")]
    pub hudcolor_xyco: TextColor,
    #[serde(default = "default_hudcolor_mapstat_title")]
    pub hudcolor_mapstat_title: TextColor,
    #[serde(default = "default_hudcolor_mapstat_value")]
    pub hudcolor_mapstat_value: TextColor,
    #[serde(default = "default_hudcolor_mapstat_time")]
    pub hudcolor_mapstat_time: TextColor,
    #[serde(default = "default_hudcolor_mesg")]
    pub hudcolor_mesg: TextColor,
    #[serde(default = "default_hudcolor_chat")]
    pub hudcolor_chat: TextColor,
    #[serde(default = "default_hudcolor_list")]
    pub hudcolor_list: TextColor,
    #[serde(default = "default_hud_msg_lines")]
    pub hud_msg_lines: MessageLines,
    #[serde(default = "default_hud_list_bgon")]
    pub hud_list_bgon: bool,
    #[serde(default = "default_health_red")]
    pub health_red: DoublePercentage,
    #[serde(default = "default_health_yellow")]
    pub health_yellow: DoublePercentage,
    #[serde(default = "default_health_green")]
    pub health_green: DoublePercentage,
    #[serde(default = "default_armor_red")]
    pub armor_red: DoublePercentage,
    #[serde(default = "default_armor_yellow")]
    pub armor_yellow: DoublePercentage,
    #[serde(default = "default_armor_green")]
    pub armor_green: DoublePercentage,
    #[serde(default = "default_ammo_red")]
    pub ammo_red: Percentage,
    #[serde(default = "default_ammo_yellow")]
    pub ammo_yellow: Percentage,
    #[serde(default = "default_ammo_colour_behaviour")]
    pub ammo_colour_behaviour: AmmoColorBehavior,
    #[serde(default = "default_hud_num")]
    pub hud_num: HudNum,
    #[serde(default = "default_hud_displayed")]
    pub hud_displayed: bool,
    #[serde(default = "default_key_speedup")]
    pub key_speedup: Keycode,
    #[serde(default = "default_key_speeddown")]
    pub key_speeddown: Keycode,
    #[serde(default = "default_key_speeddefault")]
    pub key_speeddefault: Keycode,
    #[serde(default = "default_speed_step")]
    pub speed_step: SpeedStep,
    #[serde(default = "default_key_demo_skip")]
    pub key_demo_skip: Keycode,
    #[serde(default = "default_key_level_restart")]
    pub key_level_restart: Keycode,
    #[serde(default = "default_key_nextlevel")]
    pub key_nextlevel: Keycode,
    #[serde(default = "default_key_demo_jointogame")]
    pub key_demo_jointogame: Keycode,
    #[serde(default = "default_key_demo_endlevel")]
    pub key_demo_endlevel: Keycode,
    #[serde(default = "default_key_walkcamera")]
    pub key_walkcamera: Keycode,
    #[serde(default = "default_key_showalive")]
    pub key_showalive: Keycode,
    #[serde(default = "default_hudadd_gamespeed")]
    pub hudadd_gamespeed: bool,
    #[serde(default = "default_hudadd_leveltime")]
    pub hudadd_leveltime: bool,
    #[serde(default = "default_hudadd_demotime")]
    pub hudadd_demotime: bool,
    #[serde(default = "default_hudadd_secretarea")]
    pub hudadd_secretarea: bool,
    #[serde(default = "default_hudadd_smarttotals")]
    pub hudadd_smarttotals: bool,
    #[serde(default = "default_hudadd_demoprogressbar")]
    pub hudadd_demoprogressbar: bool,
    #[serde(default = "default_hudadd_crosshair")]
    pub hudadd_crosshair: Crosshair,
    #[serde(default = "default_hudadd_crosshair_scale")]
    pub hudadd_crosshair_scale: bool,
    #[serde(default = "default_hudadd_crosshair_color")]
    pub hudadd_crosshair_color: TextColor,
    #[serde(default = "default_hudadd_crosshair_health")]
    pub hudadd_crosshair_health: bool,
    #[serde(default = "default_hudadd_crosshair_target")]
    pub hudadd_crosshair_target: bool,
    #[serde(default = "default_hudadd_crosshair_target_color")]
    pub hudadd_crosshair_target_color: TextColor,
    #[serde(default = "default_hudadd_crosshair_lock_target")]
    pub hudadd_crosshair_lock_target: bool,
    #[serde(default = "default_mouse_acceleration")]
    pub mouse_acceleration: PositiveInt,
    #[serde(default = "default_mouse_sensitivity_mlook")]
    pub mouse_sensitivity_mlook: PositiveInt,
    #[serde(default = "default_mouse_doubleclick_as_use")]
    pub mouse_doubleclick_as_use: bool,
    #[serde(default = "default_mouse_carrytics")]
    pub mouse_carrytics: bool,
    #[serde(default = "default_demo_extendedformat")]
    pub demo_extendedformat: bool,
    #[serde(default = "default_demo_demoex_filename")]
    pub demo_demoex_filename: String,
    #[serde(default = "default_getwad_cmdline")]
    pub getwad_cmdline: String,
    #[serde(default = "default_demo_overwriteexisting")]
    pub demo_overwriteexisting: bool,
    #[serde(default = "default_quickstart_window_ms")]
    pub quickstart_window_ms: Milliseconds,
    #[serde(default = "default_movement_strafe50")]
    pub movement_strafe50: bool,
    #[serde(default = "default_movement_shorttics")]
    pub movement_shorttics: bool,
    #[serde(default = "default_interpolation_maxobjects")]
    pub interpolation_maxobjects: PositiveInt,
    #[serde(default = "default_showendoom")]
    pub showendoom: bool,
    #[serde(default = "default_screenshot_dir")]
    pub screenshot_dir: String,
    #[serde(default = "default_health_bar")]
    pub health_bar: bool,
    #[serde(default = "default_health_bar_full_length")]
    pub health_bar_full_length: bool,
    #[serde(default = "default_health_bar_red")]
    pub health_bar_red: Percentage,
    #[serde(default = "default_health_bar_yellow")]
    pub health_bar_yellow: Percentage,
    #[serde(default = "default_health_bar_green")]
    pub health_bar_green: Percentage,
    #[serde(default = "default_cap_soundcommand")]
    pub cap_soundcommand: String,
    #[serde(default = "default_cap_videocommand")]
    pub cap_videocommand: String,
    #[serde(default = "default_cap_muxcommand")]
    pub cap_muxcommand: String,
    #[serde(default = "default_cap_tempfile1")]
    pub cap_tempfile1: String,
    #[serde(default = "default_cap_tempfile2")]
    pub cap_tempfile2: String,
    #[serde(default = "default_cap_remove_tempfiles")]
    pub cap_remove_tempfiles: bool,
    #[serde(default = "default_cap_fps")]
    pub cap_fps: CapFps,
    #[serde(default = "default_sdl_video_window_pos")]
    pub sdl_video_window_pos: String,
    #[serde(default = "default_palette_ondamage")]
    pub palette_ondamage: bool,
    #[serde(default = "default_palette_onbonus")]
    pub palette_onbonus: bool,
    #[serde(default = "default_palette_onpowers")]
    pub palette_onpowers: bool,
    #[serde(default = "default_render_wipescreen")]
    pub render_wipescreen: bool,
    #[serde(default = "default_render_screen_multiply")]
    pub render_screen_multiply: ScreenFactor,
    #[serde(default = "default_render_aspect")]
    pub render_aspect: AspectRatio,
    #[serde(default = "default_render_doom_lightmaps")]
    pub render_doom_lightmaps: bool,
    #[serde(default = "default_fake_contrast")]
    pub fake_contrast: bool,
    #[serde(default = "default_render_stretch_hud")]
    pub render_stretch_hud: PatchStretch,
    #[serde(default = "default_render_patches_scalex")]
    pub render_patches_scalex: PatchScale,
    #[serde(default = "default_render_patches_scaley")]
    pub render_patches_scaley: PatchScale,
    #[serde(default = "default_render_stretchsky")]
    pub render_stretchsky: bool,
    #[serde(default = "default_sprites_doom_order")]
    pub sprites_doom_order: SpriteDoomOrder,
    #[serde(default = "default_movement_mouselook")]
    pub movement_mouselook: bool,
    #[serde(default = "default_movement_maxviewpitch")]
    pub movement_maxviewpitch: RightAngle,
    #[serde(default = "default_movement_mousestrafedivisor")]
    pub movement_mousestrafedivisor: MouseStrafeDivisor,
    #[serde(default = "default_movement_mouseinvert")]
    pub movement_mouseinvert: bool,
    #[serde(default = "default_gl_allow_detail_textures")]
    pub gl_allow_detail_textures: bool,
    #[serde(default = "default_gl_detail_maxdist")]
    pub gl_detail_maxdist: DetailMaxDistance,
    #[serde(default = "default_render_multisampling")]
    pub render_multisampling: MultisamplingLevel,
    #[serde(default = "default_render_fov")]
    pub render_fov: Fov,
    #[serde(default = "default_gl_spriteclip")]
    pub gl_spriteclip: SpriteClip,
    #[serde(default = "default_gl_spriteclip_threshold")]
    pub gl_spriteclip_threshold: Percentage,
    #[serde(default = "default_gl_sprites_frustum_culling")]
    pub gl_sprites_frustum_culling: bool,
    #[serde(default = "default_render_paperitems")]
    pub render_paperitems: bool,
    #[serde(default = "default_gl_boom_colormaps")]
    pub gl_boom_colormaps: bool,
    #[serde(default = "default_gl_hires_24bit_colormap")]
    pub gl_hires_24bit_colormap: bool,
    #[serde(default = "default_gl_texture_internal_hires")]
    pub gl_texture_internal_hires: bool,
    #[serde(default = "default_gl_texture_external_hires")]
    pub gl_texture_external_hires: bool,
    #[serde(default = "default_gl_hires_override_pwads")]
    pub gl_hires_override_pwads: bool,
    #[serde(default = "default_gl_texture_hires_dir")]
    pub gl_texture_hires_dir: String,
    #[serde(default = "default_gl_texture_hqresize")]
    pub gl_texture_hqresize: bool,
    #[serde(default = "default_gl_texture_hqresize_textures")]
    pub gl_texture_hqresize_textures: HqResizeMode,
    #[serde(default = "default_gl_texture_hqresize_sprites")]
    pub gl_texture_hqresize_sprites: HqResizeMode,
    #[serde(default = "default_gl_texture_hqresize_patches")]
    pub gl_texture_hqresize_patches: HqResizeMode,
    #[serde(default = "default_gl_motionblur")]
    pub gl_motionblur: bool,
    #[serde(default = "default_gl_motionblur_min_speed")]
    pub gl_motionblur_min_speed: f64,
    #[serde(default = "default_gl_motionblur_min_angle")]
    pub gl_motionblur_min_angle: f64,
    #[serde(default = "default_gl_motionblur_att_a")]
    pub gl_motionblur_att_a: f64,
    #[serde(default = "default_gl_motionblur_att_b")]
    pub gl_motionblur_att_b: f64,
    #[serde(default = "default_gl_motionblur_att_c")]
    pub gl_motionblur_att_c: f64,
    #[serde(default = "default_gl_lightmode")]
    pub gl_lightmode: LightMode,
    #[serde(default = "default_gl_light_ambient")]
    pub gl_light_ambient: AmbientLight,
    #[serde(default = "default_gl_fog")]
    pub gl_fog: bool,
    #[serde(default = "default_gl_fog_color")]
    pub gl_fog_color: Color,
    #[serde(default = "default_useglgamma")]
    pub useglgamma: GlGamma,
    #[serde(default = "default_gl_color_mip_levels")]
    pub gl_color_mip_levels: bool,
    #[serde(default = "default_gl_shadows")]
    pub gl_shadows: bool,
    #[serde(default = "default_gl_shadows_maxdist")]
    pub gl_shadows_maxdist: ShadowMaxDistance,
    #[serde(default = "default_gl_shadows_factor")]
    pub gl_shadows_factor: ShadowFactor,
    #[serde(default = "default_gl_blend_animations")]
    pub gl_blend_animations: bool,
    #[serde(default = "default_overrun_spechit_warn")]
    pub overrun_spechit_warn: bool,
    #[serde(default = "default_overrun_spechit_emulate")]
    pub overrun_spechit_emulate: bool,
    #[serde(default = "default_overrun_reject_warn")]
    pub overrun_reject_warn: bool,
    #[serde(default = "default_overrun_reject_emulate")]
    pub overrun_reject_emulate: bool,
    #[serde(default = "default_overrun_intercept_warn")]
    pub overrun_intercept_warn: bool,
    #[serde(default = "default_overrun_intercept_emulate")]
    pub overrun_intercept_emulate: bool,
    #[serde(default = "default_overrun_playeringame_warn")]
    pub overrun_playeringame_warn: bool,
    #[serde(default = "default_overrun_playeringame_emulate")]
    pub overrun_playeringame_emulate: bool,
    #[serde(default = "default_overrun_donut_warn")]
    pub overrun_donut_warn: bool,
    #[serde(default = "default_overrun_donut_emulate")]
    pub overrun_donut_emulate: bool,
    #[serde(default = "default_overrun_missedbackside_warn")]
    pub overrun_missedbackside_warn: bool,
    #[serde(default = "default_overrun_missedbackside_emulate")]
    pub overrun_missedbackside_emulate: bool,
    #[serde(default = "default_comperr_zerotag")]
    pub comperr_zerotag: bool,
    #[serde(default = "default_comperr_passuse")]
    pub comperr_passuse: bool,
    #[serde(default = "default_comperr_hangsolid")]
    pub comperr_hangsolid: bool,
    #[serde(default = "default_comperr_blockmap")]
    pub comperr_blockmap: bool,
    #[serde(default = "default_comperr_allowjump")]
    pub comperr_allowjump: bool,
    #[serde(default = "default_comperr_freeaim")]
    pub comperr_freeaim: bool,
    #[serde(default = "default_launcher_enable")]
    pub launcher_enable: LauncherEnable,
    #[serde(default = "default_launcher_history")]
    pub launcher_history: Vec<String>,
    #[serde(default = "default_demo_patterns_mask")]
    pub demo_patterns_mask: Vec<String>,
    #[serde(default = "default_weapon_choice_1")]
    pub weapon_choice_1: Weapon,
    #[serde(default = "default_weapon_choice_2")]
    pub weapon_choice_2: Weapon,
    #[serde(default = "default_weapon_choice_3")]
    pub weapon_choice_3: Weapon,
    #[serde(default = "default_weapon_choice_4")]
    pub weapon_choice_4: Weapon,
    #[serde(default = "default_weapon_choice_5")]
    pub weapon_choice_5: Weapon,
    #[serde(default = "default_weapon_choice_6")]
    pub weapon_choice_6: Weapon,
    #[serde(default = "default_weapon_choice_7")]
    pub weapon_choice_7: Weapon,
    #[serde(default = "default_weapon_choice_8")]
    pub weapon_choice_8: Weapon,
    #[serde(default = "default_weapon_choice_9")]
    pub weapon_choice_9: Weapon,
    #[serde(default = "default_mus_e1m1")]
    pub mus_e1m1: String,
    #[serde(default = "default_mus_e1m2")]
    pub mus_e1m2: String,
    #[serde(default = "default_mus_e1m3")]
    pub mus_e1m3: String,
    #[serde(default = "default_mus_e1m4")]
    pub mus_e1m4: String,
    #[serde(default = "default_mus_e1m5")]
    pub mus_e1m5: String,
    #[serde(default = "default_mus_e1m6")]
    pub mus_e1m6: String,
    #[serde(default = "default_mus_e1m7")]
    pub mus_e1m7: String,
    #[serde(default = "default_mus_e1m8")]
    pub mus_e1m8: String,
    #[serde(default = "default_mus_e1m9")]
    pub mus_e1m9: String,
    #[serde(default = "default_mus_e2m1")]
    pub mus_e2m1: String,
    #[serde(default = "default_mus_e2m2")]
    pub mus_e2m2: String,
    #[serde(default = "default_mus_e2m3")]
    pub mus_e2m3: String,
    #[serde(default = "default_mus_e2m4")]
    pub mus_e2m4: String,
    #[serde(default = "default_mus_e2m5")]
    pub mus_e2m5: String,
    #[serde(default = "default_mus_e2m6")]
    pub mus_e2m6: String,
    #[serde(default = "default_mus_e2m7")]
    pub mus_e2m7: String,
    #[serde(default = "default_mus_e2m8")]
    pub mus_e2m8: String,
    #[serde(default = "default_mus_e2m9")]
    pub mus_e2m9: String,
    #[serde(default = "default_mus_e3m1")]
    pub mus_e3m1: String,
    #[serde(default = "default_mus_e3m2")]
    pub mus_e3m2: String,
    #[serde(default = "default_mus_e3m3")]
    pub mus_e3m3: String,
    #[serde(default = "default_mus_e3m4")]
    pub mus_e3m4: String,
    #[serde(default = "default_mus_e3m5")]
    pub mus_e3m5: String,
    #[serde(default = "default_mus_e3m6")]
    pub mus_e3m6: String,
    #[serde(default = "default_mus_e3m7")]
    pub mus_e3m7: String,
    #[serde(default = "default_mus_e3m8")]
    pub mus_e3m8: String,
    #[serde(default = "default_mus_e3m9")]
    pub mus_e3m9: String,
    #[serde(default = "default_mus_inter")]
    pub mus_inter: String,
    #[serde(default = "default_mus_intro")]
    pub mus_intro: String,
    #[serde(default = "default_mus_bunny")]
    pub mus_bunny: String,
    #[serde(default = "default_mus_victor")]
    pub mus_victor: String,
    #[serde(default = "default_mus_introa")]
    pub mus_introa: String,
    #[serde(default = "default_mus_runnin")]
    pub mus_runnin: String,
    #[serde(default = "default_mus_stalks")]
    pub mus_stalks: String,
    #[serde(default = "default_mus_countd")]
    pub mus_countd: String,
    #[serde(default = "default_mus_betwee")]
    pub mus_betwee: String,
    #[serde(default = "default_mus_doom")]
    pub mus_doom: String,
    #[serde(default = "default_mus_the_da")]
    pub mus_the_da: String,
    #[serde(default = "default_mus_shawn")]
    pub mus_shawn: String,
    #[serde(default = "default_mus_ddtblu")]
    pub mus_ddtblu: String,
    #[serde(default = "default_mus_in_cit")]
    pub mus_in_cit: String,
    #[serde(default = "default_mus_dead")]
    pub mus_dead: String,
    #[serde(default = "default_mus_stlks2")]
    pub mus_stlks2: String,
    #[serde(default = "default_mus_theda2")]
    pub mus_theda2: String,
    #[serde(default = "default_mus_doom2")]
    pub mus_doom2: String,
    #[serde(default = "default_mus_ddtbl2")]
    pub mus_ddtbl2: String,
    #[serde(default = "default_mus_runni2")]
    pub mus_runni2: String,
    #[serde(default = "default_mus_dead2")]
    pub mus_dead2: String,
    #[serde(default = "default_mus_stlks3")]
    pub mus_stlks3: String,
    #[serde(default = "default_mus_romero")]
    pub mus_romero: String,
    #[serde(default = "default_mus_shawn2")]
    pub mus_shawn2: String,
    #[serde(default = "default_mus_messag")]
    pub mus_messag: String,
    #[serde(default = "default_mus_count2")]
    pub mus_count2: String,
    #[serde(default = "default_mus_ddtbl3")]
    pub mus_ddtbl3: String,
    #[serde(default = "default_mus_ampie")]
    pub mus_ampie: String,
    #[serde(default = "default_mus_theda3")]
    pub mus_theda3: String,
    #[serde(default = "default_mus_adrian")]
    pub mus_adrian: String,
    #[serde(default = "default_mus_messg2")]
    pub mus_messg2: String,
    #[serde(default = "default_mus_romer2")]
    pub mus_romer2: String,
    #[serde(default = "default_mus_tense")]
    pub mus_tense: String,
    #[serde(default = "default_mus_shawn3")]
    pub mus_shawn3: String,
    #[serde(default = "default_mus_openin")]
    pub mus_openin: String,
    #[serde(default = "default_mus_evil")]
    pub mus_evil: String,
    #[serde(default = "default_mus_ultima")]
    pub mus_ultima: String,
    #[serde(default = "default_mus_read_m")]
    pub mus_read_m: String,
    #[serde(default = "default_mus_dm2ttl")]
    pub mus_dm2ttl: String,
    #[serde(default = "default_mus_dm2int")]
    pub mus_dm2int: String,
}

fn default_process_priority() -> ProcessPriority {
    ProcessPriority::new(0).unwrap()
}

fn default_default_compatibility_level() -> CompatibilityLevel {
    CompatibilityLevel::PrBoomLatest
}

fn default_realtic_clock_rate() -> PositiveInt {
    PositiveInt::new(100).unwrap()
}

fn default_menu_background() -> bool {
    true
}

fn default_body_queue_size() -> OptionalLimit {
    OptionalLimit::Limit(PositiveInt::new(32).unwrap())
}

fn default_flashing_hom() -> bool {
    false
}

fn default_demo_insurance() -> DemoInsurance {
    DemoInsurance::DuringDemoRecording
}

fn default_endoom_mode() -> EndoomMode {
    EndoomMode {
        colors: true,
        non_ascii_chars: false,
        skip_last_line: true,
    }
}

fn default_level_precache() -> bool {
    true
}

fn default_demo_smoothturns() -> DemoSmoothTurns {
    DemoSmoothTurns {
        enabled: true,
        factor: SmoothTurnsFactor::new(6).unwrap(),
    }
}

fn default_boom_autoswitch() -> bool {
    true
}

fn default_wad_files() -> Vec<PathBuf> {
    vec![]
}

fn default_deh_files() -> Vec<PathBuf> {
    vec![]
}

fn default_default_skill() -> SkillLevel {
    SkillLevel::Hmp
}

fn default_weapon_recoil() -> bool {
    false
}

fn default_doom_weapon_toggles() -> bool {
    true
}

fn default_player_bobbing() -> bool {
    true
}

fn default_weapon_attack_alignment() -> WeaponAttackAlignment {
    WeaponAttackAlignment::new(0).unwrap()
}

fn default_monsters_remember() -> bool {
    true
}

fn default_monster_infighting() -> MonsterInfightingLevel {
    MonsterInfightingLevel::OtherSpecies
}

fn default_monster_backing() -> bool {
    false
}

fn default_monster_avoid_hazards() -> bool {
    true
}

fn default_monkeys() -> bool {
    false
}

fn default_monster_friction() -> bool {
    true
}

fn default_help_friends() -> bool {
    false
}

fn default_allow_pushers() -> bool {
    true
}

fn default_variable_friction() -> bool {
    true
}

fn default_player_helpers() -> PlayerHelpers {
    PlayerHelpers::new(0).unwrap()
}

fn default_friend_distance() -> FriendDistance {
    FriendDistance::new(128).unwrap()
}

fn default_dog_jumping() -> bool {
    true
}

fn default_sts_always_red() -> bool {
    true
}

fn default_sts_pct_always_gray() -> bool {
    false
}

fn default_sts_traditional_keys() -> bool {
    false
}

fn default_sts_armorcolor_type() -> bool {
    true
}

fn default_show_messages() -> bool {
    true
}

fn default_autorun() -> bool {
    true
}

fn default_deh_apply_cheats() -> bool {
    true
}

fn default_comp_zombie() -> bool {
    true
}

fn default_comp_infcheat() -> bool {
    false
}

fn default_comp_stairs() -> bool {
    false
}
fn default_comp_telefrag() -> bool {
    false
}
fn default_comp_dropoff() -> bool {
    false
}
fn default_comp_falloff() -> bool {
    false
}
fn default_comp_staylift() -> bool {
    false
}
fn default_comp_doorstuck() -> bool {
    false
}
fn default_comp_pursuit() -> bool {
    false
}
fn default_comp_vile() -> bool {
    false
}
fn default_comp_pain() -> bool {
    false
}
fn default_comp_skull() -> bool {
    false
}
fn default_comp_blazing() -> bool {
    false
}
fn default_comp_doorlight() -> bool {
    false
}
fn default_comp_god() -> bool {
    false
}
fn default_comp_skymap() -> bool {
    false
}
fn default_comp_floors() -> bool {
    false
}
fn default_comp_model() -> bool {
    false
}
fn default_comp_zerotags() -> bool {
    false
}
fn default_comp_moveblock() -> bool {
    false
}
fn default_comp_sound() -> bool {
    false
}
fn default_comp_666() -> bool {
    false
}
fn default_comp_soul() -> bool {
    false
}
fn default_comp_maskedanim() -> bool {
    false
}
fn default_comp_ouchface() -> bool {
    false
}
fn default_comp_maxhealth() -> bool {
    false
}
fn default_comp_translucency() -> bool {
    false
}
fn default_snd_pcspeaker() -> bool {
    false
}
fn default_sound_card() -> SoundCard {
    SoundCard::AutoDetect
}
fn default_music_card() -> MusicCard {
    MusicCard::AutoDetect
}
fn default_pitched_sounds() -> bool {
    false
}
fn default_samplerate() -> SampleRate {
    SampleRate::new(44100).unwrap()
}
fn default_sfx_volume() -> Volume {
    Volume::new(8).unwrap()
}
fn default_music_volume() -> Volume {
    Volume::new(8).unwrap()
}
fn default_mus_pause_opt() -> MusicPauseOption {
    MusicPauseOption::ContinueWhenPaused
}
fn default_snd_channels() -> SoundChannels {
    SoundChannels::new(32).unwrap()
}
fn default_snd_midiplayer() -> MidiPlayer {
    MidiPlayer::Opl
}
fn default_snd_soundfont() -> String {
    String::from("TimGM6mb.sf2")
}
fn default_snd_mididev() -> String {
    String::from("")
}
fn default_mus_extend_volume() -> bool {
    false
}
fn default_mus_fluidsynth_chorus() -> bool {
    false
}
fn default_mus_fluidsynth_reverb() -> bool {
    false
}
fn default_mus_fluidsynth_gain() -> Gain {
    Gain::new(50).unwrap()
}
fn default_mus_opl_gain() -> Gain {
    Gain::new(50).unwrap()
}
fn default_videomode() -> VideoMode {
    VideoMode::ModeGL
}
fn default_screen_resolution() -> ScreenResolution {
    ScreenResolution {
        width: 640,
        height: 480,
    }
}
fn default_use_fullscreen() -> bool {
    false
}
fn default_render_vsync() -> bool {
    true
}
fn default_translucency() -> bool {
    true
}
fn default_tran_filter_pct() -> Percentage {
    Percentage::new(66).unwrap()
}
fn default_screenblocks() -> Screenblocks {
    Screenblocks::new(10).unwrap()
}
fn default_usegamma() -> GammaCorrectionLevel {
    GammaCorrectionLevel::new(0).unwrap()
}
fn default_uncapped_framerate() -> bool {
    true
}
fn default_test_interpolation_method() -> InterpolationMethod {
    InterpolationMethod::Fixme0
}
fn default_filter_wall() -> Filter {
    Filter::Point
}
fn default_filter_floor() -> Filter {
    Filter::Point
}
fn default_filter_sprite() -> Filter {
    Filter::Point
}
fn default_filter_z() -> ZFilter {
    ZFilter::Point
}
fn default_filter_patch() -> Filter {
    Filter::Point
}
fn default_filter_threshold() -> PositiveInt {
    PositiveInt::new(49152).unwrap()
}
fn default_sprite_edges() -> SlopedEdgeType {
    SlopedEdgeType::Square
}
fn default_patch_edges() -> SlopedEdgeType {
    SlopedEdgeType::Square
}
fn default_gl_compatibility() -> bool {
    false
}
fn default_gl_arb_multitexture() -> bool {
    true
}
fn default_gl_arb_texture_compression() -> bool {
    true
}
fn default_gl_arb_texture_non_power_of_two() -> bool {
    true
}
fn default_gl_ext_arb_vertex_buffer_object() -> bool {
    true
}
fn default_gl_arb_pixel_buffer_object() -> bool {
    true
}
fn default_gl_arb_shader_objects() -> bool {
    true
}
fn default_gl_ext_blend_color() -> bool {
    true
}
fn default_gl_ext_framebuffer_object() -> bool {
    true
}
fn default_gl_ext_packed_depth_stencil() -> bool {
    true
}
fn default_gl_ext_texture_filter_anisotropic() -> bool {
    true
}
fn default_gl_use_stencil() -> bool {
    true
}
fn default_gl_use_display_lists() -> bool {
    false
}
fn default_gl_finish() -> bool {
    true
}
fn default_gl_clear() -> bool {
    false
}
fn default_gl_ztrick() -> bool {
    false
}
fn default_gl_nearclip() -> PositiveInt {
    PositiveInt::new(5).unwrap()
}
fn default_gl_colorbuffer_bits() -> BufferBits {
    BufferBits::new(32).unwrap()
}
fn default_gl_depthbuffer_bits() -> BufferBits {
    BufferBits::new(24).unwrap()
}
fn default_gl_texture_filter() -> TextureFilter {
    TextureFilter::NearestMipmapLinear
}
fn default_gl_sprite_filter() -> SpriteFilter {
    SpriteFilter::NearestMipmapLinear
}
fn default_gl_patch_filter() -> PatchFilter {
    PatchFilter::Nearest
}
fn default_gl_texture_filter_anisotropic() -> AnisotropicFilter {
    AnisotropicFilter::On16x
}
fn default_gl_tex_format_string() -> String {
    String::from("GL_RGBA")
}
fn default_gl_sprite_offset() -> SpriteOffset {
    SpriteOffset::new(0).unwrap()
}
fn default_gl_sprite_blend() -> bool {
    false
}
fn default_gl_mask_sprite_threshold() -> Percentage {
    Percentage::new(50).unwrap()
}
fn default_gl_skymode() -> SkyType {
    SkyType::Auto
}
fn default_gl_sky_detail() -> SkyDetail {
    SkyDetail::new(16).unwrap()
}
fn default_gl_use_paletted_texture() -> bool {
    false
}
fn default_gl_use_shared_texture_palette() -> bool {
    false
}
fn default_use_mouse() -> bool {
    true
}
fn default_mouse_sensitivity_horiz() -> PositiveInt {
    PositiveInt::new(10).unwrap()
}
fn default_mouse_sensitivity_vert() -> PositiveInt {
    PositiveInt::new(0).unwrap()
}
fn default_mouseb_fire() -> MouseButton {
    MouseButton::Button0
}
fn default_mouseb_strafe() -> MouseButton {
    MouseButton::Button1
}
fn default_mouseb_forward() -> MouseButton {
    MouseButton::Button2
}
fn default_mouseb_backward() -> MouseButton {
    MouseButton::None
}
fn default_mouseb_use() -> MouseButton {
    MouseButton::None
}
fn default_key_right() -> Keycode {
    Key::RIGHTARROW
}
fn default_key_left() -> Keycode {
    Key::LEFTARROW
}
fn default_key_up() -> Keycode {
    Key::W
}
fn default_key_down() -> Keycode {
    Key::S
}
fn default_key_mlook() -> Keycode {
    Key::BACKSLASH
}
fn default_key_menu_right() -> Keycode {
    Key::RIGHTARROW
}
fn default_key_menu_left() -> Keycode {
    Key::LEFTARROW
}
fn default_key_menu_up() -> Keycode {
    Key::UPARROW
}
fn default_key_menu_down() -> Keycode {
    Key::DOWNARROW
}
fn default_key_menu_backspace() -> Keycode {
    Key::BACKSPACE
}
fn default_key_menu_escape() -> Keycode {
    Key::ESCAPE
}
fn default_key_menu_enter() -> Keycode {
    Key::ENTER
}
fn default_key_menu_clear() -> Keycode {
    Key::DEL
}
fn default_key_setup() -> Keycode {
    Key::NONE
}
fn default_key_strafeleft() -> Keycode {
    Key::A
}
fn default_key_straferight() -> Keycode {
    Key::D
}
fn default_key_flyup() -> Keycode {
    Key::PERIOD
}
fn default_key_flydown() -> Keycode {
    Key::COMMA
}
fn default_key_fire() -> Keycode {
    Key::RCTRL
}
fn default_key_use() -> Keycode {
    Key::SPACEBAR
}
fn default_key_strafe() -> Keycode {
    Key::RALT
}
fn default_key_speed() -> Keycode {
    Key::RSHIFT
}
fn default_key_savegame() -> Keycode {
    Key::F2
}
fn default_key_loadgame() -> Keycode {
    Key::F3
}
fn default_key_soundvolume() -> Keycode {
    Key::F4
}
fn default_key_hud() -> Keycode {
    Key::F5
}
fn default_key_quicksave() -> Keycode {
    Key::F6
}
fn default_key_endgame() -> Keycode {
    Key::F7
}
fn default_key_messages() -> Keycode {
    Key::F8
}
fn default_key_quickload() -> Keycode {
    Key::F9
}
fn default_key_quit() -> Keycode {
    Key::F10
}
fn default_key_gamma() -> Keycode {
    Key::F11
}
fn default_key_spy() -> Keycode {
    Key::F12
}
fn default_key_pause() -> Keycode {
    Key::PAUSE
}
fn default_key_autorun() -> Keycode {
    Key::CAPSLOCK
}
fn default_key_chat() -> Keycode {
    Key::T
}
fn default_key_backspace() -> Keycode {
    Key::BACKSPACE
}
fn default_key_enter() -> Keycode {
    Key::ENTER
}
fn default_key_map() -> Keycode {
    Key::TAB
}
fn default_key_map_right() -> Keycode {
    Key::RIGHTARROW
}
fn default_key_map_left() -> Keycode {
    Key::LEFTARROW
}
fn default_key_map_up() -> Keycode {
    Key::UPARROW
}
fn default_key_map_down() -> Keycode {
    Key::DOWNARROW
}
fn default_key_map_zoomin() -> Keycode {
    Key::EQUALS
}
fn default_key_map_zoomout() -> Keycode {
    Key::MINUS
}
fn default_key_map_gobig() -> Keycode {
    Key::ZERO
}
fn default_key_map_follow() -> Keycode {
    Key::F
}
fn default_key_map_mark() -> Keycode {
    Key::M
}
fn default_key_map_clear() -> Keycode {
    Key::C
}
fn default_key_map_grid() -> Keycode {
    Key::G
}
fn default_key_map_rotate() -> Keycode {
    Key::R
}
fn default_key_map_overlay() -> Keycode {
    Key::O
}
fn default_key_map_textured() -> Keycode {
    Key::NONE
}
fn default_key_reverse() -> Keycode {
    Key::SLASH
}
fn default_key_zoomin() -> Keycode {
    Key::EQUALS
}
fn default_key_zoomout() -> Keycode {
    Key::MINUS
}
fn default_key_chatplayer1() -> Keycode {
    Key::G
}
fn default_key_chatplayer2() -> Keycode {
    Key::I
}
fn default_key_chatplayer3() -> Keycode {
    Key::B
}
fn default_key_chatplayer4() -> Keycode {
    Key::R
}
fn default_key_weapontoggle() -> Keycode {
    Key::ZERO
}
fn default_key_weapon1() -> Keycode {
    Key::ONE
}
fn default_key_weapon2() -> Keycode {
    Key::TWO
}
fn default_key_weapon3() -> Keycode {
    Key::THREE
}
fn default_key_weapon4() -> Keycode {
    Key::FOUR
}
fn default_key_weapon5() -> Keycode {
    Key::FIVE
}
fn default_key_weapon6() -> Keycode {
    Key::SIX
}
fn default_key_weapon7() -> Keycode {
    Key::SEVEN
}
fn default_key_weapon8() -> Keycode {
    Key::EIGHT
}
fn default_key_weapon9() -> Keycode {
    Key::NINE
}
fn default_key_nextweapon() -> Keycode {
    Key::MWHEELDOWN
}
fn default_key_prevweapon() -> Keycode {
    Key::MWHEELUP
}
fn default_key_screenshot() -> Keycode {
    Key::STAR
}
fn default_use_joystick() -> bool {
    false
}
fn default_joy_left() -> usize {
    0
}
fn default_joy_right() -> usize {
    0
}
fn default_joy_up() -> usize {
    0
}
fn default_joy_down() -> usize {
    0
}
fn default_joyb_fire() -> usize {
    0
}
fn default_joyb_strafe() -> usize {
    1
}
fn default_joyb_strafeleft() -> usize {
    4
}
fn default_joyb_straferight() -> usize {
    5
}
fn default_joyb_speed() -> usize {
    2
}
fn default_joyb_use() -> usize {
    3
}
fn default_chatmacro0() -> String {
    String::from("No")
}
fn default_chatmacro1() -> String {
    String::from("I'm ready to kick butt!")
}
fn default_chatmacro2() -> String {
    String::from("I'm OK.")
}
fn default_chatmacro3() -> String {
    String::from("I'm not looking too good!")
}
fn default_chatmacro4() -> String {
    String::from("Help!")
}
fn default_chatmacro5() -> String {
    String::from("You suck!")
}
fn default_chatmacro6() -> String {
    String::from("Next time, scumbag...")
}
fn default_chatmacro7() -> String {
    String::from("Come here!")
}
fn default_chatmacro8() -> String {
    String::from("I'll take care of it.")
}
fn default_chatmacro9() -> String {
    String::from("Yes")
}
fn default_mapcolor_back() -> u32 {
    247
}
fn default_mapcolor_grid() -> u32 {
    104
}
fn default_mapcolor_wall() -> u32 {
    23
}
fn default_mapcolor_fchg() -> u32 {
    55
}
fn default_mapcolor_cchg() -> u32 {
    215
}
fn default_mapcolor_clsd() -> u32 {
    208
}
fn default_mapcolor_rkey() -> u32 {
    175
}
fn default_mapcolor_bkey() -> u32 {
    204
}
fn default_mapcolor_ykey() -> u32 {
    231
}
fn default_mapcolor_rdor() -> u32 {
    175
}
fn default_mapcolor_bdor() -> u32 {
    204
}
fn default_mapcolor_ydor() -> u32 {
    231
}
fn default_mapcolor_tele() -> u32 {
    119
}
fn default_mapcolor_secr() -> u32 {
    252
}
fn default_mapcolor_exit() -> u32 {
    0
}
fn default_mapcolor_unsn() -> u32 {
    104
}
fn default_mapcolor_flat() -> u32 {
    88
}
fn default_mapcolor_sprt() -> u32 {
    112
}
fn default_mapcolor_item() -> u32 {
    231
}
fn default_mapcolor_hair() -> u32 {
    208
}
fn default_mapcolor_sngl() -> u32 {
    208
}
fn default_mapcolor_me() -> u32 {
    112
}
fn default_mapcolor_enemy() -> u32 {
    177
}
fn default_mapcolor_frnd() -> u32 {
    112
}
fn default_map_secret_after() -> bool {
    false
}
fn default_map_point_coord() -> bool {
    false
}
fn default_map_level_stat() -> bool {
    true
}
fn default_automapmode() -> AutomapMode {
    AutomapMode {
        active: false,
        follow: true,
        grid: false,
        overlay: false,
        rotate: false,
    }
}
fn default_map_always_updates() -> bool {
    true
}
fn default_map_grid_size() -> MapGridSize {
    MapGridSize::new(128).unwrap()
}
fn default_map_scroll_speed() -> MapScrollSpeed {
    MapScrollSpeed::new(8).unwrap()
}
fn default_map_wheel_zoom() -> bool {
    true
}
fn default_map_use_multisampling() -> bool {
    true
}
fn default_map_textured() -> bool {
    true
}
fn default_map_textured_trans() -> Percentage {
    Percentage::new(100).unwrap()
}
fn default_map_textured_overlay_trans() -> Percentage {
    Percentage::new(66).unwrap()
}
fn default_map_lines_overlay_trans() -> Percentage {
    Percentage::new(100).unwrap()
}
fn default_map_overlay_pos_x() -> XPosition {
    XPosition::new(0).unwrap()
}
fn default_map_overlay_pos_y() -> YPosition {
    YPosition::new(0).unwrap()
}
fn default_map_overlay_pos_width() -> Width {
    Width::new(320).unwrap()
}
fn default_map_overlay_pos_height() -> Height {
    Height::new(200).unwrap()
}
fn default_map_things_appearance() -> MapThingsAppearance {
    MapThingsAppearance::Icon
}
fn default_hudcolor_titl() -> TextColor {
    TextColor::Gold
}
fn default_hudcolor_xyco() -> TextColor {
    TextColor::Green
}
fn default_hudcolor_mapstat_title() -> TextColor {
    TextColor::Red
}
fn default_hudcolor_mapstat_value() -> TextColor {
    TextColor::Gray
}
fn default_hudcolor_mapstat_time() -> TextColor {
    TextColor::Gray
}
fn default_hudcolor_mesg() -> TextColor {
    TextColor::Red
}
fn default_hudcolor_chat() -> TextColor {
    TextColor::Gold
}
fn default_hudcolor_list() -> TextColor {
    TextColor::Gold
}
fn default_hud_msg_lines() -> MessageLines {
    MessageLines::new(1).unwrap()
}
fn default_hud_list_bgon() -> bool {
    false
}
fn default_health_red() -> DoublePercentage {
    DoublePercentage::new(25).unwrap()
}
fn default_health_yellow() -> DoublePercentage {
    DoublePercentage::new(50).unwrap()
}
fn default_health_green() -> DoublePercentage {
    DoublePercentage::new(100).unwrap()
}
fn default_armor_red() -> DoublePercentage {
    DoublePercentage::new(25).unwrap()
}
fn default_armor_yellow() -> DoublePercentage {
    DoublePercentage::new(50).unwrap()
}
fn default_armor_green() -> DoublePercentage {
    DoublePercentage::new(100).unwrap()
}
fn default_ammo_red() -> Percentage {
    Percentage::new(25).unwrap()
}
fn default_ammo_yellow() -> Percentage {
    Percentage::new(50).unwrap()
}
fn default_ammo_colour_behaviour() -> AmmoColorBehavior {
    AmmoColorBehavior::Yes
}
fn default_hud_num() -> HudNum {
    HudNum::new(6).unwrap()
}
fn default_hud_displayed() -> bool {
    false
}
fn default_key_speedup() -> Keycode {
    Key::NONE
}
fn default_key_speeddown() -> Keycode {
    Key::NONE
}
fn default_key_speeddefault() -> Keycode {
    Key::NONE
}
fn default_speed_step() -> SpeedStep {
    SpeedStep::new(0).unwrap()
}
fn default_key_demo_skip() -> Keycode {
    Key::INSERT
}
fn default_key_level_restart() -> Keycode {
    Key::HOME
}
fn default_key_nextlevel() -> Keycode {
    Key::PAGEDOWN
}
fn default_key_demo_jointogame() -> Keycode {
    Key::Q
}
fn default_key_demo_endlevel() -> Keycode {
    Key::END
}
fn default_key_walkcamera() -> Keycode {
    Key::KEYPAD0
}
fn default_key_showalive() -> Keycode {
    Key::KEYPADDIVIDE
}
fn default_hudadd_gamespeed() -> bool {
    false
}
fn default_hudadd_leveltime() -> bool {
    false
}
fn default_hudadd_demotime() -> bool {
    false
}
fn default_hudadd_secretarea() -> bool {
    false
}
fn default_hudadd_smarttotals() -> bool {
    false
}
fn default_hudadd_demoprogressbar() -> bool {
    true
}
fn default_hudadd_crosshair() -> Crosshair {
    Crosshair::None
}
fn default_hudadd_crosshair_scale() -> bool {
    false
}
fn default_hudadd_crosshair_color() -> TextColor {
    TextColor::Green
}
fn default_hudadd_crosshair_health() -> bool {
    false
}
fn default_hudadd_crosshair_target() -> bool {
    false
}
fn default_hudadd_crosshair_target_color() -> TextColor {
    TextColor::Blue2
}
fn default_hudadd_crosshair_lock_target() -> bool {
    false
}
fn default_mouse_acceleration() -> PositiveInt {
    PositiveInt::new(0).unwrap()
}
fn default_mouse_sensitivity_mlook() -> PositiveInt {
    PositiveInt::new(10).unwrap()
}
fn default_mouse_doubleclick_as_use() -> bool {
    false
}
fn default_mouse_carrytics() -> bool {
    false
}
fn default_demo_extendedformat() -> bool {
    true
}
fn default_demo_demoex_filename() -> String {
    String::from("")
}
fn default_getwad_cmdline() -> String {
    String::from("")
}
fn default_demo_overwriteexisting() -> bool {
    true
}
fn default_quickstart_window_ms() -> Milliseconds {
    0
}
fn default_movement_strafe50() -> bool {
    false
}
fn default_movement_shorttics() -> bool {
    false
}
fn default_interpolation_maxobjects() -> PositiveInt {
    PositiveInt::new(0).unwrap()
}
fn default_showendoom() -> bool {
    false
}
fn default_screenshot_dir() -> String {
    String::from("")
}
fn default_health_bar() -> bool {
    false
}
fn default_health_bar_full_length() -> bool {
    true
}
fn default_health_bar_red() -> Percentage {
    Percentage::new(50).unwrap()
}
fn default_health_bar_yellow() -> Percentage {
    Percentage::new(99).unwrap()
}
fn default_health_bar_green() -> Percentage {
    Percentage::new(0).unwrap()
}
fn default_cap_soundcommand() -> String {
    String::from("ffmpeg -f s16le -ar {s} -ac 2 -i - -c:a libopus -y temp_a.nut")
}
fn default_cap_videocommand() -> String {
    String::from(
        "ffmpeg -f rawvideo -pix_fmt rgb24 -r {r} -s {w}x{h} -i - -c:v libx264 -y temp_v.nut",
    )
}
fn default_cap_muxcommand() -> String {
    String::from("ffmpeg -i temp_v.nut -i temp_a.nut -c copy -y {f}")
}
fn default_cap_tempfile1() -> String {
    String::from("temp_a.nut")
}
fn default_cap_tempfile2() -> String {
    String::from("temp_v.nut")
}
fn default_cap_remove_tempfiles() -> bool {
    true
}
fn default_cap_fps() -> CapFps {
    CapFps::new(60).unwrap()
}
fn default_sdl_video_window_pos() -> String {
    String::from("center")
}
fn default_palette_ondamage() -> bool {
    true
}
fn default_palette_onbonus() -> bool {
    true
}
fn default_palette_onpowers() -> bool {
    true
}
fn default_render_wipescreen() -> bool {
    true
}
fn default_render_screen_multiply() -> ScreenFactor {
    ScreenFactor::new(1).unwrap()
}
fn default_render_aspect() -> AspectRatio {
    AspectRatio::R16x9
}
fn default_render_doom_lightmaps() -> bool {
    false
}
fn default_fake_contrast() -> bool {
    true
}
fn default_render_stretch_hud() -> PatchStretch {
    PatchStretch::StretchFull
}
fn default_render_patches_scalex() -> PatchScale {
    PatchScale::new(0).unwrap()
}
fn default_render_patches_scaley() -> PatchScale {
    PatchScale::new(0).unwrap()
}
fn default_render_stretchsky() -> bool {
    true
}
fn default_sprites_doom_order() -> SpriteDoomOrder {
    SpriteDoomOrder::Static
}
fn default_movement_mouselook() -> bool {
    false
}
fn default_movement_maxviewpitch() -> RightAngle {
    RightAngle::new(90).unwrap()
}
fn default_movement_mousestrafedivisor() -> MouseStrafeDivisor {
    MouseStrafeDivisor::new(4).unwrap()
}
fn default_movement_mouseinvert() -> bool {
    false
}
fn default_gl_allow_detail_textures() -> bool {
    true
}
fn default_gl_detail_maxdist() -> DetailMaxDistance {
    DetailMaxDistance::new(0).unwrap()
}
fn default_render_multisampling() -> MultisamplingLevel {
    MultisamplingLevel::new(0).unwrap()
}
fn default_render_fov() -> Fov {
    Fov::new(90).unwrap()
}
fn default_gl_spriteclip() -> SpriteClip {
    SpriteClip::Smart
}
fn default_gl_spriteclip_threshold() -> Percentage {
    Percentage::new(10).unwrap()
}
fn default_gl_sprites_frustum_culling() -> bool {
    true
}
fn default_render_paperitems() -> bool {
    false
}
fn default_gl_boom_colormaps() -> bool {
    true
}
fn default_gl_hires_24bit_colormap() -> bool {
    false
}
fn default_gl_texture_internal_hires() -> bool {
    true
}
fn default_gl_texture_external_hires() -> bool {
    false
}
fn default_gl_hires_override_pwads() -> bool {
    false
}
fn default_gl_texture_hires_dir() -> String {
    String::from("")
}
fn default_gl_texture_hqresize() -> bool {
    false
}
fn default_gl_texture_hqresize_textures() -> HqResizeMode {
    HqResizeMode::None
}
fn default_gl_texture_hqresize_sprites() -> HqResizeMode {
    HqResizeMode::None
}
fn default_gl_texture_hqresize_patches() -> HqResizeMode {
    HqResizeMode::None
}
fn default_gl_motionblur() -> bool {
    false
}
fn default_gl_motionblur_min_speed() -> f64 {
    21.36
}
fn default_gl_motionblur_min_angle() -> f64 {
    20.0
}
fn default_gl_motionblur_att_a() -> f64 {
    55.0
}
fn default_gl_motionblur_att_b() -> f64 {
    1.8
}
fn default_gl_motionblur_att_c() -> f64 {
    0.9
}
fn default_gl_lightmode() -> LightMode {
    LightMode::Shaders
}
fn default_gl_light_ambient() -> AmbientLight {
    AmbientLight::new(20).unwrap()
}
fn default_gl_fog() -> bool {
    true
}
fn default_gl_fog_color() -> u32 {
    0xffffff
}
fn default_useglgamma() -> GlGamma {
    GlGamma::new(6).unwrap()
}
fn default_gl_color_mip_levels() -> bool {
    false
}
fn default_gl_shadows() -> bool {
    false
}
fn default_gl_shadows_maxdist() -> ShadowMaxDistance {
    ShadowMaxDistance::new(1000).unwrap()
}
fn default_gl_shadows_factor() -> ShadowFactor {
    ShadowFactor::new(128).unwrap()
}
fn default_gl_blend_animations() -> bool {
    false
}
fn default_overrun_spechit_warn() -> bool {
    false
}
fn default_overrun_spechit_emulate() -> bool {
    true
}
fn default_overrun_reject_warn() -> bool {
    false
}
fn default_overrun_reject_emulate() -> bool {
    true
}
fn default_overrun_intercept_warn() -> bool {
    false
}
fn default_overrun_intercept_emulate() -> bool {
    true
}
fn default_overrun_playeringame_warn() -> bool {
    false
}
fn default_overrun_playeringame_emulate() -> bool {
    true
}
fn default_overrun_donut_warn() -> bool {
    false
}
fn default_overrun_donut_emulate() -> bool {
    false
}
fn default_overrun_missedbackside_warn() -> bool {
    false
}
fn default_overrun_missedbackside_emulate() -> bool {
    false
}
fn default_comperr_zerotag() -> bool {
    false
}
fn default_comperr_passuse() -> bool {
    false
}
fn default_comperr_hangsolid() -> bool {
    false
}
fn default_comperr_blockmap() -> bool {
    false
}
fn default_comperr_allowjump() -> bool {
    false
}
fn default_comperr_freeaim() -> bool {
    false
}
fn default_launcher_enable() -> LauncherEnable {
    LauncherEnable::Never
}
fn default_launcher_history() -> Vec<String> {
    vec![]
}
fn default_demo_patterns_mask() -> Vec<String> {
    vec![
        "DOOM 2: Hell on Earth/((lv)|(nm)|(pa)|(ty))\\d\\d.\\d\\d\\d\\.lmp/doom2.wad",
        "DOOM 2: Plutonia Experiment/p(c|f|l|n|p|r|s|t)\\d\\d.\\d\\d\\d\\.lmp/doom2.wad|plutonia.wad",
        "DOOM 2: TNT - Evilution/((e(c|f|v|p|r|s|t))|(tn))\\d\\d.\\d\\d\\d\\.lmp/doom2.wad|tnt.wad",
        "The Ultimate DOOM/(((e|f|n|p|r|t|u)\\dm\\d)|(n\\ds\\d)).\\d\\d\\d\\.lmp/doom.wad",
        "Alien Vendetta/a(c|f|n|p|r|s|t|v)\\d\\d.\\d\\d\\d\\.lmp/doom2.wad|av.wad|av.deh",
        "Requiem/r(c|f|n|p|q|r|s|t)\\d\\d.\\d\\d\\d\\.lmp/doom2.wad|requiem.wad|req21fix.wad|reqmus.wad",
        "Hell Revealed/h(c|e|f|n|p|r|s|t)\\d\\d.\\d\\d\\d\\.lmp/doom2.wad|hr.wad|hrmus.wad",
        "Memento Mori/mm\\d\\d.\\d\\d\\d\\.lmp/doom2.wad|mm.wad|mmmus.wad",
        "Memento Mori 2/m2\\d\\d.\\d\\d\\d\\.lmp/doom2.wad|mm2.wad|mm2mus.wad"
    ]
    .iter()
    .map(|s| String::from(*s))
    .collect()
}
fn default_weapon_choice_1() -> Weapon {
    Weapon::new(6).unwrap()
}
fn default_weapon_choice_2() -> Weapon {
    Weapon::new(9).unwrap()
}
fn default_weapon_choice_3() -> Weapon {
    Weapon::new(4).unwrap()
}
fn default_weapon_choice_4() -> Weapon {
    Weapon::new(3).unwrap()
}
fn default_weapon_choice_5() -> Weapon {
    Weapon::new(2).unwrap()
}
fn default_weapon_choice_6() -> Weapon {
    Weapon::new(8).unwrap()
}
fn default_weapon_choice_7() -> Weapon {
    Weapon::new(5).unwrap()
}
fn default_weapon_choice_8() -> Weapon {
    Weapon::new(7).unwrap()
}
fn default_weapon_choice_9() -> Weapon {
    Weapon::new(1).unwrap()
}
fn default_mus_e1m1() -> String {
    String::from("e1m1.mp3")
}
fn default_mus_e1m2() -> String {
    String::from("e1m2.mp3")
}
fn default_mus_e1m3() -> String {
    String::from("e1m3.mp3")
}
fn default_mus_e1m4() -> String {
    String::from("e1m4.mp3")
}
fn default_mus_e1m5() -> String {
    String::from("e1m5.mp3")
}
fn default_mus_e1m6() -> String {
    String::from("e1m6.mp3")
}
fn default_mus_e1m7() -> String {
    String::from("e1m7.mp3")
}
fn default_mus_e1m8() -> String {
    String::from("e1m8.mp3")
}
fn default_mus_e1m9() -> String {
    String::from("e1m9.mp3")
}
fn default_mus_e2m1() -> String {
    String::from("e2m1.mp3")
}
fn default_mus_e2m2() -> String {
    String::from("e2m2.mp3")
}
fn default_mus_e2m3() -> String {
    String::from("e2m3.mp3")
}
fn default_mus_e2m4() -> String {
    String::from("e2m4.mp3")
}
fn default_mus_e2m5() -> String {
    String::from("e2m5.mp3")
}
fn default_mus_e2m6() -> String {
    String::from("e2m6.mp3")
}
fn default_mus_e2m7() -> String {
    String::from("e2m7.mp3")
}
fn default_mus_e2m8() -> String {
    String::from("e2m8.mp3")
}
fn default_mus_e2m9() -> String {
    String::from("e2m9.mp3")
}
fn default_mus_e3m1() -> String {
    String::from("e3m1.mp3")
}
fn default_mus_e3m2() -> String {
    String::from("e3m2.mp3")
}
fn default_mus_e3m3() -> String {
    String::from("e3m3.mp3")
}
fn default_mus_e3m4() -> String {
    String::from("e3m4.mp3")
}
fn default_mus_e3m5() -> String {
    String::from("e3m5.mp3")
}
fn default_mus_e3m6() -> String {
    String::from("e3m6.mp3")
}
fn default_mus_e3m7() -> String {
    String::from("e3m7.mp3")
}
fn default_mus_e3m8() -> String {
    String::from("e3m8.mp3")
}
fn default_mus_e3m9() -> String {
    String::from("e3m9.mp3")
}
fn default_mus_inter() -> String {
    String::from("inter.mp3")
}
fn default_mus_intro() -> String {
    String::from("intro.mp3")
}
fn default_mus_bunny() -> String {
    String::from("bunny.mp3")
}
fn default_mus_victor() -> String {
    String::from("victor.mp3")
}
fn default_mus_introa() -> String {
    String::from("introa.mp3")
}
fn default_mus_runnin() -> String {
    String::from("runnin.mp3")
}
fn default_mus_stalks() -> String {
    String::from("stalks.mp3")
}
fn default_mus_countd() -> String {
    String::from("countd.mp3")
}
fn default_mus_betwee() -> String {
    String::from("betwee.mp3")
}
fn default_mus_doom() -> String {
    String::from("doom.mp3")
}
fn default_mus_the_da() -> String {
    String::from("the_da.mp3")
}
fn default_mus_shawn() -> String {
    String::from("shawn.mp3")
}
fn default_mus_ddtblu() -> String {
    String::from("ddtblu.mp3")
}
fn default_mus_in_cit() -> String {
    String::from("in_cit.mp3")
}
fn default_mus_dead() -> String {
    String::from("dead.mp3")
}
fn default_mus_stlks2() -> String {
    String::from("stlks2.mp3")
}
fn default_mus_theda2() -> String {
    String::from("theda2.mp3")
}
fn default_mus_doom2() -> String {
    String::from("doom2.mp3")
}
fn default_mus_ddtbl2() -> String {
    String::from("ddtbl2.mp3")
}
fn default_mus_runni2() -> String {
    String::from("runni2.mp3")
}
fn default_mus_dead2() -> String {
    String::from("dead2.mp3")
}
fn default_mus_stlks3() -> String {
    String::from("stlks3.mp3")
}
fn default_mus_romero() -> String {
    String::from("romero.mp3")
}
fn default_mus_shawn2() -> String {
    String::from("shawn2.mp3")
}
fn default_mus_messag() -> String {
    String::from("messag.mp3")
}
fn default_mus_count2() -> String {
    String::from("count2.mp3")
}
fn default_mus_ddtbl3() -> String {
    String::from("ddtbl3.mp3")
}
fn default_mus_ampie() -> String {
    String::from("ampie.mp3")
}
fn default_mus_theda3() -> String {
    String::from("theda3.mp3")
}
fn default_mus_adrian() -> String {
    String::from("adrian.mp3")
}
fn default_mus_messg2() -> String {
    String::from("messg2.mp3")
}
fn default_mus_romer2() -> String {
    String::from("romer2.mp3")
}
fn default_mus_tense() -> String {
    String::from("tense.mp3")
}
fn default_mus_shawn3() -> String {
    String::from("shawn3.mp3")
}
fn default_mus_openin() -> String {
    String::from("openin.mp3")
}
fn default_mus_evil() -> String {
    String::from("evil.mp3")
}
fn default_mus_ultima() -> String {
    String::from("ultima.mp3")
}
fn default_mus_read_m() -> String {
    String::from("read_m.mp3")
}
fn default_mus_dm2ttl() -> String {
    String::from("dm2ttl.mp3")
}
fn default_mus_dm2int() -> String {
    String::from("dm2int.mp3")
}

#[repr(i32)]
#[derive(
    Debug, Copy, Clone, EnumString, Serialize, Deserialize, TryFromPrimitive, PartialOrd, PartialEq,
)]
pub enum CompatibilityLevel {
    DoomV12 = 0,
    DoomV1666,
    Doom2V19,
    UltimateDoom,
    FinalDoom,
    DosDoom,
    TasDoom,
    Boom,
    BoomV201,
    BoomV202,
    LxDoomV1,
    Mbf,
    PrBoomV203Beta,
    PrBoomV210211,
    PrBoomV22x,
    PrBoomV23x,
    PrBoomV240,
    PrBoomLatest,
}

impl Default for CompatibilityLevel {
    fn default() -> Self {
        Self::PrBoomLatest
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct ProcessPriority { 0..=2 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct PositiveInt { 0..std::i32::MAX }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OptionalLimit {
    NoLimit,
    Limit(PositiveInt),
}

impl Default for OptionalLimit {
    fn default() -> Self {
        OptionalLimit::NoLimit
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum DemoInsurance {
    None,
    Always,
    DuringDemoRecording,
}

impl Default for DemoInsurance {
    fn default() -> Self {
        DemoInsurance::DuringDemoRecording
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DemoSmoothTurns {
    pub enabled: bool,
    pub factor: SmoothTurnsFactor,
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct SmoothTurnsFactor { 1..=16 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct WeaponAttackAlignment { 0..=3 }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillLevel {
    None,
    Itytd,
    Hntr,
    Hmp,
    Uv,
    Nm,
}

impl TryFrom<u8> for SkillLevel {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SkillLevel::Itytd),
            1 => Ok(SkillLevel::Hntr),
            2 => Ok(SkillLevel::Hmp),
            3 => Ok(SkillLevel::Uv),
            4 => Ok(SkillLevel::Nm),
            _ => Err(format!("Invalid skill level {}", value)),
        }
    }
}

impl Display for SkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SkillLevel::None => "None",
                SkillLevel::Itytd => "I'm Too Young To Die",
                SkillLevel::Hntr => "Hey, Not Too Rough",
                SkillLevel::Hmp => "Hurt Me Plenty",
                SkillLevel::Uv => "Ultra-Violence",
                SkillLevel::Nm => "Nightmare",
            }
        )
    }
}

impl Default for SkillLevel {
    fn default() -> Self {
        SkillLevel::Hmp
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MonsterInfightingLevel {
    None,
    OtherSpecies,
    All,
}

impl Default for MonsterInfightingLevel {
    fn default() -> Self {
        MonsterInfightingLevel::OtherSpecies
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct PlayerHelpers { 0..=3 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct FriendDistance { 0..1000 }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum SoundCard {
    AutoDetect,
    None,
    Card(i32),
}
pub type MusicCard = SoundCard;

impl Default for SoundCard {
    fn default() -> Self {
        SoundCard::AutoDetect
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct SampleRate { 11025..=48000 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Volume { 0..16 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum MusicPauseOption {
    KillWhenPaused,
    PauseWhenPaused,
    ContinueWhenPaused,
}

impl Default for MusicPauseOption {
    fn default() -> Self {
        MusicPauseOption::ContinueWhenPaused
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct SoundChannels { 1..=32 }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MidiPlayer {
    Sdl,
    Fluidsynth,
    Opl,
    PortMidi,
}

impl Default for MidiPlayer {
    fn default() -> Self {
        MidiPlayer::Opl
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Gain { 0..=1000 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Percentage { 0..=100 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct DoublePercentage { 0..=200 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Screenblocks { 3..=11 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Gamma { 0..=4 }
}

// FIXME: What are these values?
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum InterpolationMethod {
    Fixme0,
    Fixme1,
}

impl Default for InterpolationMethod {
    fn default() -> Self {
        InterpolationMethod::Fixme0
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Filter {
    None,
    Point,
    Linear,
    Rounded,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::None
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SlopedEdgeType {
    Square,
    Sloped,
}

impl Default for SlopedEdgeType {
    fn default() -> Self {
        SlopedEdgeType::Square
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct BufferBits { 16..=32 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

impl Default for TextureFilter {
    fn default() -> Self {
        TextureFilter::NearestMipmapLinear
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SpriteFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
}

impl Default for SpriteFilter {
    fn default() -> Self {
        SpriteFilter::NearestMipmapLinear
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PatchFilter {
    Nearest,
    Linear,
}

impl Default for PatchFilter {
    fn default() -> Self {
        PatchFilter::Nearest
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AnisotropicFilter {
    Off,
    On2x,
    On4x,
    On8x,
    On16x,
}

impl Default for AnisotropicFilter {
    fn default() -> Self {
        AnisotropicFilter::On16x
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EndoomMode {
    pub colors: bool,
    pub non_ascii_chars: bool,
    pub skip_last_line: bool,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SkyType {
    Auto,
    None,
    Standard,
    Skydome,
    Screen,
}

impl Default for SkyType {
    fn default() -> Self {
        SkyType::Auto
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AutomapMode {
    pub active: bool,
    pub overlay: bool,
    pub rotate: bool,
    pub follow: bool,
    pub grid: bool,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MapThingsAppearance {
    Classic,
    Scaled,
    Icon,
}

impl Default for MapThingsAppearance {
    fn default() -> Self {
        MapThingsAppearance::Icon
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AmmoColorBehavior {
    No,
    FullOnly,
    Yes,
    Max,
}

impl Default for AmmoColorBehavior {
    fn default() -> Self {
        AmmoColorBehavior::No
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PatchStretch {
    Stretch16x10,
    Stretch4x3,
    StretchFull,
    StretchMax,
}

impl Default for PatchStretch {
    fn default() -> Self {
        PatchStretch::StretchFull
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SpriteDoomOrder {
    None,
    Static,
    Dynamic,
    Last,
}

impl Default for SpriteDoomOrder {
    fn default() -> Self {
        SpriteDoomOrder::Dynamic
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SpriteClip {
    Const,
    Always,
    Smart,
}

impl Default for SpriteClip {
    fn default() -> Self {
        SpriteClip::Smart
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum HqResizeMode {
    None,
    X2,
    X3,
    X4,
    Max,
}

impl Default for HqResizeMode {
    fn default() -> Self {
        HqResizeMode::None
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum LightMode {
    GlBoom,
    GzDoom,
    FogBased,
    Shaders,
}

impl Default for LightMode {
    fn default() -> Self {
        LightMode::Shaders
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EmulationSetting {
    pub warn: bool,
    pub emulate: bool,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum VideoMode {
    Mode8,
    Mode15,
    Mode16,
    Mode32,
    ModeGL,
}

impl Default for VideoMode {
    fn default() -> Self {
        VideoMode::ModeGL
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ScreenResolution {
    pub width: usize,
    pub height: usize,
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct GammaCorrectionLevel { 0..=4 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum ZFilter {
    Point,
    Linear,
}

impl Default for ZFilter {
    fn default() -> Self {
        ZFilter::Point
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct SpriteOffset { 0..=5 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct SkyDetail { 1..=32 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum MouseButton {
    None,
    Button0,
    Button1,
    Button2,
    Button3,
    Button4,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::None
    }
}

pub type Color = u32;
pub type Keycode = Key;

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct MapGridSize { 8..=256 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct MapScrollSpeed { 1..=32 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct XPosition { 0..320 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct YPosition { 0..200 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Width { 0..=320 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Height { 0..=200 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct MessageLines { 1..=16 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum TextColor {
    Brick,
    Tan,
    Gray,
    Green,
    Brown,
    Gold,
    Red,
    Blue,
    Orange,
    Yellow,
    Blue2,
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor::Red
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct HudNum { 0..=100 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct SpeedStep { 0..=1000 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Crosshair {
    None,
    Cross,
    Angle,
    Dot,
}

impl Default for Crosshair {
    fn default() -> Self {
        Crosshair::None
    }
}

pub type Milliseconds = usize;

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct CapFps { 16..=300 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct ScreenFactor { 1..=4 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum AspectRatio {
    R16x9,
    R16x10,
    R4x3,
    R5x4,
}

impl Default for AspectRatio {
    fn default() -> Self {
        AspectRatio::R16x9
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct PatchScale { 0..=16 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct RightAngle { 0..=90 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct MouseStrafeDivisor { 1..=512 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct DetailMaxDistance { 0..65536 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct MultisamplingLevel { 0..=8 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Fov { 20..=160 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct AmbientLight { 1..256 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct GlGamma { 0..=32 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct ShadowMaxDistance { 0..32768 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct ShadowFactor { 0..256 }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum LauncherEnable {
    Never,
    Smart,
    Always,
}

impl Default for LauncherEnable {
    fn default() -> Self {
        LauncherEnable::Always
    }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Default)]
    pub struct Weapon { 0..=9 }
}
