use std::simd::f32x4;
use super::{Vec3, Versor};

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct EulerAngles (Vec3);

impl EulerAngles {
    #[inline]
    pub const fn from_radians (roll: f32, pitch: f32, yaw: f32) -> Self {
        return Self::from_radians_vec(Vec3::new(roll, pitch, yaw))
    }

    #[inline]
    pub fn from_degrees (roll: f32, pitch: f32, yaw: f32) -> Self {
        return Self::from_degrees_vec(Vec3::new(roll, pitch, yaw))
    }

    #[inline]
    pub const fn from_radians_vec (vec: Vec3) -> Self {
        return Self(vec)
    }

    #[inline]
    pub fn from_degrees_vec (vec: Vec3) -> Self {
        const WEIGHT: f32 = std::f32::consts::PI / 180.;
        return Self(vec * WEIGHT)
    }

    #[inline]
    pub const fn to_radians (self) -> Vec3 {
        return self.0
    }

    #[inline]
    pub fn to_degrees (self) -> Vec3 {
        const WEIGHT: f32 = 180. / std::f32::consts::PI;
        return self.0 * WEIGHT
    }
    
    #[inline]
    pub const fn to_inner (self) -> f32x4 {
        return self.0.to_inner()
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
        let this = self.0.to_inner() % WEIGHT;
        let other = other.0.to_inner() % WEIGHT;
        return this == other
    }
}