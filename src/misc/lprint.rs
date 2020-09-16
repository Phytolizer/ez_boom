use bitflags::bitflags;
use lazy_static::lazy_static;
use parking_lot::RwLock;

bitflags! {
    pub struct OutputLevel: u32 {
        const NONE    = 0b00000000;
        const INFO    = 0b00000001;
        const CONFIRM = 0b00000010;
        const WARN    = 0b00000100;
        const ERROR   = 0b00001000;
        const FATAL   = 0b00010000;
        const DEBUG   = 0b00100000;
        const ALWAYS  = 0b01000000;

        const ALL = Self::INFO.bits
                  | Self::CONFIRM.bits
                  | Self::WARN.bits
                  | Self::ERROR.bits
                  | Self::FATAL.bits
                  | Self::DEBUG.bits
                  | Self::ALWAYS.bits;
    }
}

lazy_static! {
    pub static ref OUTPUT_MASK: RwLock<OutputLevel> = RwLock::new(OutputLevel::ALL);
    pub static ref ERROR_MASK: RwLock<OutputLevel> =
        RwLock::new(OutputLevel::ALL & !OutputLevel::INFO);
}

#[macro_export]
macro_rules! lprint {
    ($pri:expr, $($arg:tt)*) => {
        {
            if *crate::misc::lprint::OUTPUT_MASK.read() & $pri != crate::misc::lprint::OutputLevel::NONE {
                print!($($arg)*);
            }
        }
    };
}
