pub mod sphere;
use crate::vec3::Vec3;

pub trait Object {
    fn is_hit_by(&self, ray: Ray) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct HitRecord {
    pub time: f32
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
    pub fn hits<T: Object>(self, target: &T) -> Option<HitRecord> {
        T::is_hit_by(target, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{vec3::Vec3, object::{sphere::Sphere, Ray, Object}};

    #[test]
    fn test_sphere_hit() {
        let center = Vec3::ZERO;
        let sphere = Sphere::new(center, 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.is_hit_by(ray);
        assert_eq!(hit.unwrap().time, 4.0);
    }
}
