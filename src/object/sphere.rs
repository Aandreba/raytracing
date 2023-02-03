use std::{hint::unreachable_unchecked, cmp::Ordering};
use crate::math::Vec3;
use super::{Object, Ray, HitRecord};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
}

impl Sphere {
    #[inline]
    pub const fn new (center: Vec3, radius: f32) -> Self {
        return Self { radius, center };
    }
}

// https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
impl Object for Sphere {
    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<HitRecord> {
        let dist = ray.origin - self.center;
        let alpha = ray.direction * dist;
        let delta = (alpha * alpha) - (dist.sq_norm() - (self.radius * self.radius));

        if delta.is_nan() || delta.is_infinite() || delta < 0.0 {
            return None
        }

        let time = f32::sqrt(delta) - alpha;
        return match time.partial_cmp(&0.0) {
            Some(Ordering::Greater | Ordering::Equal) => Some(HitRecord { time }),
            None => unsafe { unreachable_unchecked() },
            _ => None
        }
    }
}