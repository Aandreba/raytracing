use super::{Object, Ray};
use crate::math::{Vec3, UnitVec3};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    #[inline]
    pub const fn new(center: Vec3, radius: f32) -> Self {
        return Self { center, radius }
    }
}

// https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
impl Object for Sphere {
    #[inline]
    fn normal (&self, at: Vec3) -> UnitVec3 {
        return (at - self.center).unit()
    }

    #[inline]
    fn is_hit_by(&self, ray: Ray) -> Option<f32> {
        let dist = ray.origin - self.center;
        let alpha = ray.direction * dist;
        let delta = (alpha * alpha) - (dist.sq_norm() - (self.radius * self.radius));

        let alpha = -alpha;
        let beta = f32::sqrt(delta);

        let time = match alpha.partial_cmp(&beta)? {
            Ordering::Equal => 0.0,
            Ordering::Greater => alpha - beta,
            Ordering::Less => match alpha + beta {
                x if x < 0.0 => return None,
                x => x,
            },
        };

        return Some(time);
    }
}

#[cfg(test)]
#[test]
fn test_sphere_hit() {
    let center = Vec3::new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(center, 1.0);
    let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0).unit());
    let hit = sphere.is_hit_by(ray);

    println!("{hit:?}");
    assert_eq!(hit.unwrap(), 4.0);
}
