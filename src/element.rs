use crate::{object::{Object, Ray}, math::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct ReflectInfo {
    pub color: Vec3, // rgb
    pub ray: Ray
}

pub struct Element<T> {
    pub color: Vec3,
    pub object: T,
}

impl<T: Object> Element<T> {
    #[inline]
    pub fn new (object: T, color: Vec3) -> Self {
        return Self { object, color }
    }

    #[inline]
    pub fn into_dyn<'a> (self) -> Element<Box<dyn 'a + Object>> where T: 'a {
        return Element {
            color: self.color,
            object: Box::new(self.object)
        }
    }

    #[inline]
    pub fn interact (&self, mut prev: ReflectInfo) -> Option<ReflectInfo> {
        let _hit: f32 = prev.ray.hits(&self.object)?;
        prev.color = prev.color.wide_mul(self.color); // todo
        //prev.ray = Ray::new(hit.position, hit.position.cross(prev.ray.direction)); // todo
        return Some(prev)
    }
}

impl<'a> Element<Box<dyn 'a + Object>> {
    #[inline]
    pub fn new_unzise (object: impl 'a + Object, color: Vec3) -> Self {
        return Element::new(object, color).into_dyn()
    }
}