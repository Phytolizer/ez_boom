use super::def::{GameMission, GameMode, Language};

use lazy_static::lazy_static;
use parking_lot::RwLock;

lazy_static! {
    pub(crate) static ref GAMEMODE: RwLock<GameMode> = RwLock::new(GameMode::TBD);
    pub(crate) static ref GAMEMISSION: RwLock<GameMission> = RwLock::new(GameMission::Doom);
    pub(crate) static ref LANGUAGE: RwLock<Language> = RwLock::new(Language::English);
}
