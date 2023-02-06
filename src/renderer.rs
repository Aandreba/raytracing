use image::Rgb;
use std::borrow::Borrow;

use crate::{
    display::Framebuffer,
    element::{Element, ReflectInfo},
    light::{DynLight, Light},
    math::Vec3,
    object::{DynObject, Object, Ray},
};

pub struct Renderer<E, L> {
    frame: Framebuffer,
    pub elements: E,
    pub lights: L,
}

impl<'a, E, L> Renderer<E, L>
where
    E: Borrow<[Element<DynObject<'a>>]>,
    L: Borrow<[DynLight<'a>]>,
{
    #[inline]
    pub fn new(frame: Framebuffer, elements: E, lights: L) -> Self {
        return Self {
            elements,
            frame,
            lights,
        };
    }

    pub fn render(&mut self, depth: usize) -> anyhow::Result<()>
    where
        E: Send + Sync,
    {
        // TODO FIX
        // todo depth
        let limit = depth - 1;
        let elements: &[Element<DynObject>] = self.elements.borrow();
        let lights: &[DynLight] = self.lights.borrow();

        self.frame.update(core::convert::identity, |pos, _| {
            let mut prev_info = ReflectInfo {
                color: Vec3::ZERO,
                ray: Ray::new(Vec3::ZERO, pos.unit()),
            };

            for i in 0..depth {
                let mut result = None;
                for element in elements.iter() {
                    match (element.object.is_hit_by(prev_info.ray), result) {
                        (Some(t), None) => result = Some((element as &Element<DynObject>, t)),
                        (Some(t), Some((_, prev))) if t < prev => result = Some((element, t)),
                        _ => {}
                    }
                }

                if let Some((element, t)) = result {
                    let new_origin = prev_info.ray.position_at(t);
                    
                    let mut color = prev_info.color;
                    for light in lights.iter() {
                        if let Some(c) = Light::hits(light as &DynLight, new_origin) {
                            color += c
                        }
                    }
                    
                    prev_info.color = color.wide_mul(element.color);
                    if i < limit {
                        let normal = element.object.normal(prev_info.ray.origin);
                        prev_info.ray.direction = prev_info.ray.reflect(normal); // todo
                        prev_info.ray.origin = new_origin
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
