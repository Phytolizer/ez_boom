use lazy_static::lazy_static;
use parking_lot::RwLock;

pub(crate) struct RuntimeConfig {
    pub(crate) nomonsters: bool,
    pub(crate) respawnparm: bool,
    pub(crate) fastparm: bool,
    pub(crate) devparm: bool,

    // can also be 2
    pub(crate) deathmatch: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            nomonsters: false,
            respawnparm: false,
            fastparm: false,
            devparm: false,
            deathmatch: 0,
        }
    }
}

lazy_static! {
    pub(crate) static ref RUNTIME_CONFIG: RwLock<RuntimeConfig> =
        RwLock::new(RuntimeConfig::default());
}
