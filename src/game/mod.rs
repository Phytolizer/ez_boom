use lazy_static::lazy_static;
use parking_lot::RwLock;

lazy_static! {
    pub(crate) static ref FORWARD_MOVE: RwLock<[i32; 2]> = RwLock::new([0x19, 0x32]);
    pub(crate) static ref SIDE_MOVE: RwLock<[i32; 2]> = RwLock::new([0x18, 0x28]);
}
