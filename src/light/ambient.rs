use crate::math::Vec3;
use super::{DynLight, Light};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ambient {
    color: Vec3
}

impl Ambient {
    #[inline]
    pub const fn new (color: Vec3) -> Self {
        return Self { color }
    }

    #[inline]
    pub fn new_unsize (color: Vec3) -> DynLight<'static> {
        Box::new(Self::new(color))
    }
}

impl Light for Ambient {
    #[inline]
    fn hits (&self, _point: Vec3) -> Option<Vec3> {
        Some(self.color)
    }
}