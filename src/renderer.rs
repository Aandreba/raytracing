use bracket_color::rgb::RGB;

use crate::{
    display::Framebuffer,
    element::{Element, ReflectInfo},
    math::{Vec3, Vec2},
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

    pub fn render(&mut self, depth: usize) -> anyhow::Result<()> {
        for element in self.elements.iter() {
            self.frame.update(core::convert::identity, |pos, _| {
                let mut prev_info = ReflectInfo {
                    color: Vec3::splat(1.0),
                    ray: Ray::new(Vec3::ZERO, Vec3::from_vec2(Vec2::from(pos).unit(), 1.0)),
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

                Some(
                    RGB::from_f32(
                        prev_info.color.x(),
                        prev_info.color.y(),
                        prev_info.color.z(),
                    )
                    .to_hsv(),
                )
            });
        }

        self.frame.display()?;
        self.frame.clear();
        return Ok(());
    }
}
