use crate::{math::{UnitVec3, Vec3}};
use std::sync::Arc;
pub mod sphere;

pub type DynObject<'a> = Box<dyn 'a + Object>;

pub trait Object: Send + Sync {
    fn normal(&self, at: Vec3) -> UnitVec3;
    fn is_hit_by(&self, ray: Ray) -> Option<f32>;
}

impl<T: ?Sized + Object> Object for &T {
    #[inline]
    fn normal(&self, at: Vec3) -> UnitVec3 {
        T::normal(*self, at)
    }

    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<f32> {
        T::is_hit_by(*self, ray)
    }
}

impl<T: ?Sized + Object> Object for Box<T> {
    #[inline]
    fn normal(&self, at: Vec3) -> UnitVec3 {
        T::normal(self, at)
    }

    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<f32> {
        T::is_hit_by(self, ray)
    }
}

impl<T: ?Sized + Object> Object for Arc<T> {
    #[inline]
    fn normal(&self, at: Vec3) -> UnitVec3 {
        T::normal(self, at)
    }

    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<f32> {
        T::is_hit_by(self, ray)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: UnitVec3,
}

impl Ray {
    #[inline]
    pub const fn new(origin: Vec3, direction: UnitVec3) -> Self {
        return Self { origin, direction };
    }

    #[inline]
    pub fn position_at(self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }

    #[inline]
    pub fn hits<T: Object>(self, target: &T) -> Option<f32> {
        T::is_hit_by(target, self)
    }

    #[inline]
    pub fn reflect(self, normal: UnitVec3) -> UnitVec3 {
        Vec3::unit(self.direction - 2. * (self.direction * normal) * normal)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::{UnitVec3, Vec3},
        object::{sphere::Sphere, Object, Ray},
    };

    #[test]
    fn test_sphere_hit() {
        let center = Vec3::ZERO;
        let sphere = Sphere::new(center, 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), unsafe {
            UnitVec3::new_unchecked(0.0, 0.0, 1.0)
        });
        let hit = sphere.is_hit_by(ray);
        assert_eq!(hit.unwrap(), 4.0);
    }
}
