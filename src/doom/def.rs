use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};

/// Handle IWAD-dependent animations, &c based
/// on the value of this enum
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    TBD,
    Shareware,
    Registered,
    Commercial,
    Retail,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameMission {
    Doom,
    Doom2,
    TNT,
    Plutonia,
    Nerve,
    Hacx,
    Chex,
    None,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    German,
    Unknown,
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Key: usize {
        const RIGHTARROW = 0xae;
        const LEFTARROW = 0xac;
        const UPARROW = 0xad;
        const DOWNARROW = 0xaf;
        const ESCAPE = 27;
        const ENTER = 13;
        const TAB = 9;
        const F1 = 0x80 + 0x3b;
        const F2 = 0x80 + 0x3c;
        const F3 = 0x80 + 0x3d;
        const F4 = 0x80 + 0x3e;
        const F5 = 0x80 + 0x3f;
        const F6 = 0x80 + 0x40;
        const F7 = 0x80 + 0x41;
        const F8 = 0x80 + 0x42;
        const F9 = 0x80 + 0x43;
        const F10 = 0x80 + 0x44;
        const F11 = 0x80 + 0x57;
        const F12 = 0x80 + 0x58;
        const BACKSPACE = 127;
        const PAUSE = 0xff;
        const EQUALS = 0x3d;
        const MINUS = 0x2d;
        const RSHIFT = 0x80 + 0x36;
        const RCTRL = 0x80 + 0x1d;
        const RALT = 0x80 + 0x38;
        const LALT = Key::RALT.bits;
        const CAPSLOCK = 0xba;
        const PRINTSC = 0xfe;

        // phares 3/2/98:
        const INSERT = 0xd2;
        const HOME = 0xc7;
        const PAGEUP = 0xc9;
        const PAGEDOWN = 0xd1;
        const DEL = 0xc8;
        const END = 0xcf;
        const SCROLLLOCK = 0xc6;
        const SPACEBAR = 0x20;
        // phares 3/2/98
        const NUMLOCK = 0xC5;
        // cph - Add the numeric keypad keys; as suggested by krose 4/22/99:
        // The way numbers are assigned to keys is a mess; but it's too late to
        // change that easily. At least these additions are don neatly.
        // Codes 0x100-0x200 are reserved for number pad
        const KEYPAD0 = (0x100 + b'0' as usize);
        const KEYPAD1 = (0x100 + b'1' as usize);
        const KEYPAD2 = (0x100 + b'2' as usize);
        const KEYPAD3 = (0x100 + b'3' as usize);
        const KEYPAD4 = (0x100 + b'4' as usize);
        const KEYPAD5 = (0x100 + b'5' as usize);
        const KEYPAD6 = (0x100 + b'6' as usize);
        const KEYPAD7 = (0x100 + b'7' as usize);
        const KEYPAD8 = (0x100 + b'8' as usize);
        const KEYPAD9 = (0x100 + b'9' as usize);
        const KEYPADENTER = 0x100 + Key::ENTER.bits;
        const KEYPADDIVIDE = (0x100 + b'/' as usize);
        const KEYPADMULTIPLY = (0x100 + b'*' as usize);
        const KEYPADMINUS = (0x100 + b'-' as usize);
        const KEYPADPLUS = (0x100 + b'+' as usize);
        const KEYPADPERIOD = (0x100 + b'.' as usize);

        // haleyjd: virtual keys
        const MOUSE1 = 0x80 + 0x60;
        const MOUSE2 = 0x80 + 0x61;
        const MOUSE3 = 0x80 + 0x62;
        const MWHEELUP = 0x80 + 0x6b;
        const MWHEELDOWN = 0x80 + 0x6c;

        const ZERO = b'0' as usize;
        const ONE = b'1' as usize;
        const TWO = b'2' as usize;
        const THREE = b'3' as usize;
        const FOUR = b'4' as usize;
        const FIVE = b'5' as usize;
        const SIX = b'6' as usize;
        const SEVEN = b'7' as usize;
        const EIGHT = b'8' as usize;
        const NINE = b'9' as usize;
        const W = b'w' as usize;
        const S = b's' as usize;
        const A = b'a' as usize;
        const D = b'd' as usize;
        const T = b't' as usize;
        const F = b'f' as usize;
        const M = b'm' as usize;
        const C = b'c' as usize;
        const G = b'g' as usize;
        const R = b'r' as usize;
        const O = b'o' as usize;
        const I = b'i' as usize;
        const B = b'b' as usize;
        const Q = b'q' as usize;
        const SLASH = b'/' as usize;
        const STAR = b'*' as usize;
        const NONE = 0;
        const BACKSLASH = b'\\' as usize;
        const PERIOD = b'.' as usize;
        const COMMA = b',' as usize;
    }
}

impl Default for Key {
    fn default() -> Self {
        Key::NONE
    }
}
