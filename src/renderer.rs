use image::Rgb;

use crate::{
    display::Framebuffer,
    element::{Element, ReflectInfo},
    math::Vec3,
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
    pub fn push<'b, T: 'a + Object>(
        &'b mut self,
        element: Element<T>,
    ) -> &'b mut Element<Box<dyn 'a + Object>>
    where
        'a: 'b,
    {
        self.push_boxed(element.into_dyn())
    }

    #[inline]
    pub fn push_boxed<'b>(
        &'b mut self,
        element: Element<Box<dyn 'a + Object>>,
    ) -> &'b mut Element<Box<dyn 'a + Object>>
    where
        'a: 'b,
    {
        let idx = self.elements.len();
        self.elements.push(element);
        return unsafe { self.elements.get_unchecked_mut(idx) };
    }

    pub fn render(&mut self, depth: usize) -> anyhow::Result<()> {
        // TODO depth
        self.frame.update(core::convert::identity, |pos, _| {
            let prev_info = ReflectInfo {
                color: Vec3::splat(1.0),
                ray: Ray::new(Vec3::ZERO, pos),
            };
            let mut result = None;
            for element in self.elements.iter() {
                match (element.object.is_hit_by(prev_info.ray), result) {
                    (Some(t), None) => result = Some((element, t)),
                    (Some(t), Some((_, prev))) if t < prev => result = Some((element, t)),
                    _ => {}
                }
            }

            let (element, _time) = result?;
            Some(Rgb(element.color.to_array().map(|x| x as u8)))
        });

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

                let color = Vec3::to_inner(255. * prev_info.color).cast::<u8>();
                unsafe { Some(Rgb(*(color.as_array() as *const [u8; 4] as *const [u8; 3]))) }
            });
        }

        self.frame.display()?;
        self.frame.clear();
        return Ok(());
    }
}
