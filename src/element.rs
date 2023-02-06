use crate::{object::{Object, Ray}, math::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct ReflectInfo {
    pub color: Vec3, // rgb
    pub ray: Ray
}

pub struct Element<T> {
    pub object: T,
    pub material: Material,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Vec3,
    pub reflectiveness: Vec3
}

impl<T: Object> Element<T> {
    #[inline]
    pub fn new (object: T, material: Material) -> Self {
        return Self { object, material }
    }

    #[inline]
    pub fn into_dyn<'a> (self) -> Element<Box<dyn 'a + Object>> where T: 'a {
        return Element {
            material: self.material,
            object: Box::new(self.object)
        }
    }
}

impl<'a> Element<Box<dyn 'a + Object>> {
    #[inline]
    pub fn new_unzise (object: impl 'a + Object, material: Material) -> Self {
        return Element::new(object, material).into_dyn()
    }
}

impl Material {
    #[inline]
    pub const fn new (color: Vec3, reflectiveness: Vec3) -> Self {
        Self { color, reflectiveness }
    }
}