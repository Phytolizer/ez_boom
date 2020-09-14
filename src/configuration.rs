#![allow(dead_code)]

use bounded_integer::bounded_integer;
use ops::Range;
use serde_derive::{Deserialize, Serialize};
use std::{convert::TryFrom, env, fmt::Display, ops, path::PathBuf};
use strum_macros::EnumString;

use crate::{
    doom::def::GameMission,
    doom::def::GameMode,
    doom::def::Language,
    doom_exe_dir, misc,
    misc::args::{ArgMeta, Args},
    wad::WadFileInfo,
};
use misc::ConfigParam;

#[derive(Debug)]
pub(crate) struct Configuration {
    pub(crate) defaults: Box<Defaults>,

    pub(crate) args: Args,
    pub(crate) arg_meta: ArgMeta,
    pub(crate) nomonsters: bool,
    pub(crate) respawnparm: bool,
    pub(crate) fastparm: bool,
    pub(crate) devparm: bool,

    // can also be 2
    pub(crate) deathmatch: usize,
    pub(crate) force_old_bsp: bool,

    pub(crate) game_mode: GameMode,
    pub(crate) game_mission: GameMission,
    pub(crate) language: Language,

    pub(crate) doom_ver_str: String,
    pub(crate) bfg_edition: bool,
    pub(crate) has_wolf_levels: bool,

    pub(crate) save_game_base: PathBuf,
    pub(crate) start_skill: SkillLevel,
    pub(crate) start_episode: usize,
    pub(crate) start_map: usize,
    pub(crate) autostart: bool,

    pub(crate) wad_files: Vec<WadFileInfo>,

    pub(crate) forward_move: [i32; 2],
    pub(crate) side_move: [i32; 2],

    pub(crate) no_music: bool,
    pub(crate) no_sfx: bool,

    pub(crate) no_draw: bool,
    pub(crate) no_blit: bool,

    pub(crate) view_angle_offset: i32,

    pub(crate) default_file: PathBuf,

    pub(crate) weapon_recoil: bool,
    pub(crate) player_bobbing: bool,
    pub(crate) variable_friction: bool,
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

            weapon_recoil: defaults.weapon_recoil,
            player_bobbing: defaults.player_bobbing,
            variable_friction: defaults.variable_friction,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct Defaults {
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
}

// impl Default for Defaults {
//     fn default() -> Self {
//         Self {
//             process_priority: default_process_priority(),
//             default_compatibility_level: default_default_compatibility_level(),
//             realtic_clock_rate: default_realtic_clock_rate(),
//             menu_background: default_menu_background(),
//             body_queue_size: default_body_queue_size(),
//             flashing_hom: default_flashing_hom(),
//             demo_insurance: default_demo_insurance(),
//             endoom_mode: default_endoom_mode(),
//             level_precache: default_level_precache(),
//             demo_smoothturns: default_demo_smoothturns(),
//             boom_autoswitch: default_boom_autoswitch(),
//             wad_files: default_wad_files(),
//             deh_files: default_deh_files(),
//             default_skill: default_default_skill(),
//             weapon_recoil: default_weapon_recoil(),
//             doom_weapon_toggles: default_doom_weapon_toggles(),
//             player_bobbing: default_player_bobbing(),
//             weapon_attack_alignment: default_weapon_attack_alignment(),
//             monsters_remember: default_monsters_remember(),
//             monster_infighting: default_monster_infighting(),
//             monster_backing: default_monster_backing(),
//             monster_avoid_hazards: default_monster_avoid_hazards(),
//             monkeys: default_monkeys(),
//             monster_friction: default_monster_friction(),
//             help_friends: default_help_friends(),
//             allow_pushers: default_allow_pushers(),
//             variable_friction: default_variable_friction(),
//             player_helpers: default_player_helpers(),
//             friend_distance: default_friend_distance(),
//             dog_jumping: default_dog_jumping(),
//             sts_always_red: default_sts_always_red(),
//             sts_pct_always_gray: default_sts_pct_always_gray(),
//         }
//     }
// }

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

#[derive(Debug, Copy, Clone, EnumString, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    DoomV12,
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
    #[derive(Serialize, Deserialize, Default)]
    pub struct ProcessPriority { 0..=2 }
}

bounded_integer! {
    #[repr(i32)]
    #[derive(Serialize, Deserialize, Default)]
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
    #[derive(Serialize, Deserialize, Default)]
    pub struct SmoothTurnsFactor { 1..=16 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Serialize, Deserialize, Default)]
    pub struct WeaponAttackAlignment { 0..=3 }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
    #[derive(Serialize, Deserialize, Default)]
    pub struct PlayerHelpers { 0..=3 }
}
bounded_integer! {
    #[repr(i32)]
    #[derive(Serialize, Deserialize, Default)]
    pub struct FriendDistance { 0..1000 }
}

pub enum SoundCard {
    AutoDetect,
    None,
    Card(i32),
}
pub type MusicCard = SoundCard;

bounded_integer! {
    #[repr(i32)]
    pub struct SampleRate { 11025..=48000 }
}
bounded_integer! {
    #[repr(i32)]
    pub struct Volume { 0..16 }
}

pub enum MusicPauseOption {
    KillWhenPaused,
    PauseWhenPaused,
    ContinueWhenPaused,
}

bounded_integer! {
    #[repr(i32)]
    pub struct SoundChannels { 1..=32 }
}

#[derive(PartialEq, Eq, Debug)]
pub enum MidiPlayer {
    Sdl,
    Fluidsynth,
    Opl,
    PortMidi,
}

bounded_integer! {
    #[repr(i32)]
    pub struct Gain { 0..=1000 }
}
bounded_integer! {
    #[repr(i32)]
    pub struct Percentage { 0..=100 }
}
bounded_integer! {
    #[repr(i32)]
    pub struct Screenblocks { 3..=11 }
}
bounded_integer! {
    #[repr(i32)]
    pub struct Gamma { 0..=4 }
}

// FIXME: What are these values?
pub enum InterpolationMethod {
    Fixme0,
    Fixme1,
}

pub enum Filter {
    None,
    Point,
    Linear,
    Rounded,
}

pub enum SlopedEdgeType {
    Square,
    Sloped,
}

bounded_integer! {
    #[repr(i32)]
    pub struct BufferBits { 16..=32 }
}

pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

pub enum SpriteFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
}

pub enum PatchFilter {
    Nearest,
    Linear,
}

pub enum AnisotropicFilter {
    Off,
    On2x,
    On4x,
    On8x,
    On16x,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EndoomMode {
    pub colors: bool,
    pub non_ascii_chars: bool,
    pub skip_last_line: bool,
}

pub enum SkyType {
    Auto,
    None,
    Standard,
    Skydome,
    Screen,
}

pub struct AutomapMode {
    pub active: bool,
    pub overlay: bool,
    pub rotate: bool,
    pub follow: bool,
    pub grid: bool,
}

pub enum MapThingsAppearance {
    Classic,
    Scaled,
    Icon,
}

pub enum AmmoColorBehavior {
    No,
    FullOnly,
    Yes,
    Max,
}

pub enum PatchStretch {
    Stretch16x10,
    Stretch4x3,
    StretchFull,
    StretchMax,
}

pub enum SpriteDoomOrder {
    None,
    Static,
    Dynamic,
    Last,
}

pub enum SpriteClip {
    Const,
    Always,
    Smart,
}

pub enum HqResizeMode {
    None,
    Some2x,
    Some3x,
    Some4x,
    SomeMax,
}

pub enum LightMode {
    GlBoom,
    GzDoom,
    FogBased,
    Shaders,
}

pub struct EmulationSetting {
    pub warn: bool,
    pub emulate: bool,
}
