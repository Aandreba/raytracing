use crate::{math::Vec3};
use super::{Light, DynLight};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub point: Vec3,
    pub color: Vec3,
    pub intensity: f32
}

impl Point {
    #[inline]
    pub const fn new (point: Vec3, color: Vec3, intensity: f32) -> Self {
        return Self { point, color, intensity }
    }

    #[inline]
    pub fn new_unsize (point: Vec3, color: Vec3, intensity: f32) -> DynLight<'static> {
        Box::new(Self::new(point, color, intensity))
    }
}

impl Light for Point {
    #[inline]
    fn hits (&self, point: Vec3) -> Option<Vec3> {
        let dist = point.distance(self.point);
        let intensity = self.intensity / (dist * dist);
        if intensity <= f32::EPSILON { return None; }
        return Some(intensity * self.color)
    }
}