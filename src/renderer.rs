use std::{borrow::Borrow};
use image::Rgb;

use crate::{
    display::Framebuffer,
    element::{Element, ReflectInfo},
    math::Vec3,
    object::{Object, Ray, DynObject},
};

pub struct Renderer<E> {
    frame: Framebuffer,
    pub elements: E,
}

impl<'a, E: Borrow<[Element<DynObject<'a>>]>> Renderer<E> {
    #[inline]
    pub fn new(frame: Framebuffer, elements: E) -> Self {
        return Self { elements, frame };
    }

    pub fn render(&mut self, depth: usize) -> anyhow::Result<()> where E: Send + Sync {
        // TODO FIX
        // todo depth
        let limit = depth - 1;
        let element: &[Element<DynObject>] = self.elements.borrow();

        self.frame.update(core::convert::identity, |pos, _| {
            let mut prev_info = ReflectInfo {
                color: Vec3::splat(1.0),
                ray: Ray::new(Vec3::ZERO, pos),
            };

            for i in 0..depth {
                let mut result = None;
                for element in self.elements.borrow().iter() {
                    match (element.object.is_hit_by(prev_info.ray), result) {
                        (Some(t), None) => result = Some((element, t)),
                        (Some(t), Some((_, prev))) if t < prev => result = Some((element, t)),
                        _ => {}
                    }
                }

                if let Some((element, t)) = result {
                    let normal = element

                    prev_info.color = prev_info.color.wide_mul(element.color);
                    if i < limit {
                        prev_info.ray.origin = prev_info.ray.position_at(t);
                        prev_info.ray.direction = Vec3::splat(1.).unit(); // todo
                    }
                }
            }

            let color = 255. * prev_info.color;
            Rgb(color.to_array().map(|x| x as u8))
        });

        self.frame.display()?;
        self.frame.clear();
        return Ok(());
    }
}
