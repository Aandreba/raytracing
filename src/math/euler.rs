use std::simd::f32x4;
use super::{Vec3, Versor};

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct EulerAngles (Vec3);

impl EulerAngles {
    #[inline]
    pub const fn new (roll: f32, pitch: f32, yaw: f32) -> Self {
        return Self(Vec3::new(roll, pitch, yaw))
    }

    #[inline]
    pub fn from_angles (roll: f32, pitch: f32, yaw: f32) -> Self {
        const WEIGHT: f32 = std::f32::consts::PI / 180.;
        return Self::from_vec(WEIGHT * Vec3::new(roll, pitch, yaw))
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
    pub fn to_versor (self) -> Versor {
        Versor::from_euler(self)
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

impl PartialEq for EulerAngles {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        const WEIGHT: f32x4 = f32x4::from_array([std::f32::consts::TAU; 4]);
        let this = self.0.into_inner() % WEIGHT;
        let other = other.0.into_inner() % WEIGHT;
        return this == other
    }
}