#![allow(dead_code)]

pub const ANG45: usize = 0x20000000;
pub const ANG90: usize = 0x40000000;
pub const ANG180: usize = 0x80000000;
pub const ANG270: usize = 0xc0000000;
pub const ANG1: usize = ANG45 / 45;
pub const ANGLE_MAX: usize = 0xffffffff;
