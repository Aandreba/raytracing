use image::Rgb;

use crate::{
    display::Framebuffer,
    element::{Element, ReflectInfo},
    math::{Vec3},
    object::{Object, Ray},
};

pub struct Renderer<'a> {
    frame: Framebuffer,
    pub elements: Vec<Element<Box<dyn 'a + Object>>>,
}

impl<'a> Renderer<'a> {
    #[inline]
    pub fn new(frame: Framebuffer, elements: Vec<Element<Box<dyn 'a + Object>>>) -> Self {
        return Self { elements, frame };
    }

    #[inline]
    pub fn push<'b, T: 'a + Object> (&'b mut self, element: Element<T>) -> &'b mut Element<Box<dyn 'a + Object>> where 'a: 'b {
        self.push_boxed(element.into_dyn())
    }

    #[inline]
    pub fn push_boxed<'b> (&'b mut self, element: Element<Box<dyn 'a + Object>>) -> &'b mut Element<Box<dyn 'a + Object>> where 'a: 'b {
        let idx = self.elements.len();
        self.elements.push(element);
        return unsafe { self.elements.get_unchecked_mut(idx) }
    }

    pub fn render(&mut self, depth: usize) -> anyhow::Result<()> {
        for element in self.elements.iter() {
            self.frame.update(core::convert::identity, |pos, _| {
                let mut prev_info = ReflectInfo {
                    color: Vec3::splat(1.0),
                    ray: Ray::new(Vec3::ZERO, pos),
                };

                for i in 0..depth {
                    if let Some(info) = element.interact(prev_info) {
                        prev_info = info;
                        continue;
                    } else if i == 0 {
                        prev_info.color = Vec3::ZERO;
                    }
                    break;
                }

                Some(Rgb(prev_info.color.to_array()))
            });
        }

        self.frame.display()?;
        self.frame.clear();
        return Ok(());
    }
}
