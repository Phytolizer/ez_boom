#![allow(dead_code)]

pub(crate) const ANG45: usize = 0x20000000;
pub(crate) const ANG90: usize = 0x40000000;
pub(crate) const ANG180: usize = 0x80000000;
pub(crate) const ANG270: usize = 0xc0000000;
pub(crate) const ANG1: usize = ANG45 / 45;
pub(crate) const ANGLE_MAX: usize = 0xffffffff;
pub(crate) const PI: f64 = 3.14159265358979323846;
