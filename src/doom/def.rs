use serde_derive::{Deserialize, Serialize};

/// Handle IWAD-dependent animations, &c based
/// on the value of this enum
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum GameMode {
    TBD,
    Shareware,
    Registered,
    Commercial,
    Retail,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum GameMission {
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
pub(crate) enum Language {
    English,
    French,
    German,
    Unknown,
}
