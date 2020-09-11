#![allow(dead_code)]

use bounded_integer::bounded_integer;
use num::clamp;
use std::ops;

pub struct Configuration {}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {}
    }
}

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

bounded_integer! {
    #[repr(i32)]
    pub struct ProcessPriority { 0..=2 }
}
bounded_integer! {
    #[repr(i32)]
    pub struct PositiveInt { 0..std::i32::MAX }
}

pub enum OptionalLimit {
    NoLimit,
    Limit(PositiveInt),
}

pub enum DemoInsurance {
    None,
    Always,
    DuringDemoRecording,
}

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

pub enum SkillLevel {
    Itytd,
    Hntr,
    Hmp,
    Uv,
    Nm,
}

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

pub struct BoundedInt {
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

pub type HexInt = BoundedInt;
pub type KeyInt = BoundedInt;
pub type MouseButton = BoundedInt;
pub type Color = BoundedInt;

impl BoundedInt {
    pub fn new(value: i32, min: i32, max: i32) -> Self {
        Self { value, min, max }
    }

    pub fn set(&mut self, value: i32) {
        self.value = clamp(value, self.min, self.max);
    }
}

impl ops::Add for BoundedInt {
    type Output = i32;
    fn add(self, rhs: Self) -> Self::Output {
        self.value + rhs.value
    }
}

impl ops::AddAssign for BoundedInt {
    fn add_assign(&mut self, rhs: Self) {
        let value = self.value.saturating_add(rhs.value);
        self.value = clamp(value, self.min, self.max);
    }
}

impl ops::BitAnd for BoundedInt {
    type Output = i32;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.value & rhs.value
    }
}

impl ops::BitAndAssign for BoundedInt {
    fn bitand_assign(&mut self, rhs: Self) {
        let value = self.value & rhs.value;
        self.value = clamp(value, self.min, self.max);
    }
}

impl ops::BitOr for BoundedInt {
    type Output = i32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.value | rhs.value
    }
}

impl ops::BitOrAssign for BoundedInt {
    fn bitor_assign(&mut self, rhs: Self) {
        let value = self.value | rhs.value;
        self.value = clamp(value, self.min, self.max);
    }
}

impl ops::BitXor for BoundedInt {
    type Output = i32;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.value ^ rhs.value
    }
}

impl ops::BitXorAssign for BoundedInt {
    fn bitxor_assign(&mut self, rhs: Self) {
        let value = self.value ^ rhs.value;
        self.value = clamp(value, self.min, self.max);
    }
}

impl ops::Div for BoundedInt {
    type Output = i32;
    fn div(self, rhs: Self) -> Self::Output {
        self.value / rhs.value
    }
}

impl ops::DivAssign for BoundedInt {
    fn div_assign(&mut self, rhs: Self) {
        let value = self.value / rhs.value;
        self.value = clamp(value, self.min, self.max);
    }
}

impl ops::Mul for BoundedInt {
    type Output = i32;
    fn mul(self, rhs: Self) -> Self::Output {
        self.value * rhs.value
    }
}

impl ops::MulAssign for BoundedInt {
    fn mul_assign(&mut self, rhs: Self) {
        let value = self.value * rhs.value;
        self.value = clamp(value, self.min, self.max);
    }
}

impl ops::Sub for BoundedInt {
    type Output = i32;
    fn sub(self, rhs: Self) -> Self::Output {
        self.value - rhs.value
    }
}

impl ops::SubAssign for BoundedInt {
    fn sub_assign(&mut self, rhs: Self) {
        let value = self.value - rhs.value;
        self.value = clamp(value, self.min, self.max);
    }
}

impl From<BoundedInt> for i32 {
    fn from(i: BoundedInt) -> Self {
        i.value
    }
}
