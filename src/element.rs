use bracket_color::{rgb::RGB};
use crate::object::Object;

pub struct Element<T> {
    pub material: Material,
    pub object: T,
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

pub struct Material {
    color: RGB
}