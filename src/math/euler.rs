use std::simd::f32x4;
use super::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct EulerAngles (Vec3);

impl EulerAngles {
    #[inline]
    pub const fn new (roll: f32, pitch: f32, yaw: f32) -> Self {
        return Self(Vec3::new(roll, pitch, yaw))
    }

    #[inline]
    pub const fn from_vec (vec: Vec3) -> Self {
        return Self(vec)
    }

    #[inline]
    pub const fn into_vec (self) -> Vec3 {
        return self.0
    }
    
    #[inline]
    pub const fn into_inner (self) -> f32x4 {
        return self.0.into_inner()
    }

    #[inline]
    pub fn roll (self) -> f32 {
        self.0.x()
    }

    #[inline]
    pub fn pitch (self) -> f32 {
        self.0.y()
    }

    #[inline]
    pub fn yaw (self) -> f32 {
        self.0.z()
    }
}