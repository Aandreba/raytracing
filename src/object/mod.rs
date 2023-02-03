use std::{sync::Arc};
use crate::math::Vec3;
pub mod sphere;

pub trait Object: Send + Sync {
    fn is_hit_by(&self, ray: Ray) -> Option<HitInfo>;
}

impl<T: ?Sized + Object> Object for &T {
    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<HitInfo> {
        T::is_hit_by(*self, ray)
    }
}

impl<T: ?Sized + Object> Object for Box<T> {
    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<HitInfo> {
        T::is_hit_by(self, ray)
    }
}

impl<T: ?Sized + Object> Object for Arc<T> {
    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<HitInfo> {
        T::is_hit_by(self, ray)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct HitInfo {
    pub time: f32,
    pub position: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // unit vector
}

impl Ray {
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        unsafe { return Self::new_unchecked(origin, direction.unit()) }
    }

    #[inline]
    pub const unsafe fn new_unchecked(origin: Vec3, direction: Vec3) -> Self {
        return Self { origin, direction };
    }

    #[inline]
    pub fn hits<T: Object>(self, target: &T) -> Option<HitInfo> {
        T::is_hit_by(target, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::Vec3,
        object::{sphere::Sphere, Object, Ray},
    };

    #[test]
    fn test_sphere_hit() {
        let center = Vec3::ZERO;
        let sphere = Sphere::new(center, 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.is_hit_by(ray);
        assert_eq!(hit.unwrap().time, 4.0);
    }
}
