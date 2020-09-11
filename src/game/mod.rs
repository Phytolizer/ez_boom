use lazy_static::lazy_static;
use parking_lot::RwLock;

lazy_static! {
    pub(crate) static ref HAS_WOLF_LEVELS: RwLock<bool> = RwLock::new(false);
}
