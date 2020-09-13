#![allow(dead_code)]

use bounded_integer::bounded_integer;
use ops::Range;
use std::{convert::TryFrom, env, fmt::Display, ops, path::PathBuf};

use crate::{
    doom::def::GameMission,
    doom::def::GameMode,
    doom::def::Language,
    doom_exe_dir, misc,
    misc::args::{ArgMeta, Args},
    wad::WadFileInfo,
};

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

            weapon_recoil: defaults.weapon_recoil.value,
            player_bobbing: defaults.player_bobbing.value,
            variable_friction: defaults.variable_friction.value,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Defaults {
    pub process_priority: DefaultValue<ProcessPriority>,
    pub default_compatibility_level: DefaultValue<CompatibilityLevel>,
    pub realtic_clock_rate: DefaultValue<PositiveInt>,
    pub menu_background: DefaultValue<bool>,
    pub body_queue_size: DefaultValue<OptionalLimit>,
    pub flashing_hom: DefaultValue<bool>,
    pub demo_insurance: DefaultValue<DemoInsurance>,
    pub endoom_mode: DefaultValue<EndoomMode>,
    pub level_precache: DefaultValue<bool>,
    pub demo_smoothturns: DefaultValue<DemoSmoothTurns>,
    pub boom_autoswitch: DefaultValue<bool>,
    pub wad_files: DefaultValue<Vec<PathBuf>>,
    pub deh_files: DefaultValue<Vec<PathBuf>>,
    pub default_skill: DefaultValue<SkillLevel>,
    pub weapon_recoil: DefaultValue<bool>,
    pub doom_weapon_toggles: DefaultValue<bool>,
    pub player_bobbing: DefaultValue<bool>,
    pub weapon_attack_alignment: DefaultValue<WeaponAttackAlignment>,
    pub monsters_remember: DefaultValue<bool>,
    pub monster_infighting: DefaultValue<MonsterInfightingLevel>,
    pub monster_backing: DefaultValue<bool>,
    pub monster_avoid_hazards: DefaultValue<bool>,
    pub monkeys: DefaultValue<bool>,
    pub monster_friction: DefaultValue<bool>,
    pub help_friends: DefaultValue<bool>,
    pub allow_pushers: DefaultValue<bool>,
    pub variable_friction: DefaultValue<bool>,
    pub player_helpers: DefaultValue<PlayerHelpers>,
    pub friend_distance: DefaultValue<FriendDistance>,
    pub dog_jumping: DefaultValue<bool>,
    pub sts_always_red: DefaultValue<bool>,
    pub sts_pct_always_gray: DefaultValue<bool>,
}

macro_rules! default_values {
    (
        $sname:tt {
            $($name:ident: $value:expr,)*
        }
    ) => {
        $sname {
            $($name : DefaultValue{name: stringify!($name), value:$value},)*
        }
    };
}

impl Default for Defaults {
    fn default() -> Self {
        default_values!(Self {
            process_priority: ProcessPriority::new(0).unwrap(),
            default_compatibility_level: CompatibilityLevel::PrBoomLatest,
            realtic_clock_rate: PositiveInt::new(100).unwrap(),
            menu_background: true,
            body_queue_size: OptionalLimit::Limit(PositiveInt::new(32).unwrap()),
            flashing_hom: false,
            demo_insurance: DemoInsurance::DuringDemoRecording,
            endoom_mode: EndoomMode {
                colors: true,
                non_ascii_chars: false,
                skip_last_line: true,
            },
            level_precache: true,
            demo_smoothturns: DemoSmoothTurns {
                enabled: true,
                factor: SmoothTurnsFactor::new(6).unwrap(),
            },
            boom_autoswitch: true,
            wad_files: vec![],
            deh_files: vec![],
            default_skill: SkillLevel::Hmp,
            weapon_recoil: false,
            doom_weapon_toggles: true,
            player_bobbing: true,
            weapon_attack_alignment: WeaponAttackAlignment::new(0).unwrap(),
            monsters_remember: true,
            monster_infighting: MonsterInfightingLevel::OtherSpecies,
            monster_backing: false,
            monster_avoid_hazards: true,
            monkeys: false,
            monster_friction: true,
            help_friends: false,
            allow_pushers: true,
            variable_friction: true,
            player_helpers: PlayerHelpers::new(0).unwrap(),
            friend_distance: FriendDistance::new(128).unwrap(),
            dog_jumping: true,
            sts_always_red: true,
            sts_pct_always_gray: false,
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefaultValue<T> {
    pub name: &'static str,
    pub value: T,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
pub struct BoundedInt<T>
where
    T: PartialOrd + Copy,
{
    bounds: Range<T>,
    value: T,
}

impl<T> BoundedInt<T>
where
    T: PartialOrd + Copy,
{
    pub fn new(bounds: Range<T>, value: T) -> BoundedInt<T> {
        if !bounds.contains(&value) {
            panic!("value out of range");
        }
        Self { bounds, value }
    }

    pub fn set(&mut self, value: T) -> Result<(), ()> {
        if self.bounds.contains(&value) {
            self.value = value;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get(&self) -> T {
        self.value
    }
}

bounded_integer! {
    #[repr(i32)]
    pub struct ProcessPriority { 0..=2 }
}

impl Default for ProcessPriority {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

bounded_integer! {
    #[repr(i32)]
    pub struct PositiveInt { 0..std::i32::MAX }
}

#[derive(Debug)]
pub enum OptionalLimit {
    NoLimit,
    Limit(PositiveInt),
}

#[derive(Debug, Copy, Clone)]
pub enum DemoInsurance {
    None,
    Always,
    DuringDemoRecording,
}

#[derive(Debug)]
pub struct DemoSmoothTurns {
    pub enabled: bool,
    pub factor: SmoothTurnsFactor,
}

bounded_integer! {
    #[repr(i32)]
    pub struct SmoothTurnsFactor { 1..=16 }
}
bounded_integer! {
    #[repr(i32)]
    pub struct WeaponAttackAlignment { 0..=3 }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum MonsterInfightingLevel {
    None,
    OtherSpecies,
    All,
}

bounded_integer! {
    #[repr(i32)]
    pub struct PlayerHelpers { 0..=3 }
}
bounded_integer! {
    #[repr(i32)]
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

#[derive(Debug)]
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
