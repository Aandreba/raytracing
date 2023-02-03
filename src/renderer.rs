use crate::{display::Framebuffer, element::Element, object::Object};

pub struct Renderer {
    frame: Framebuffer,
    elements: Vec<Element<Box<dyn Object>>>
}

impl Renderer {
    #[inline]
    fn render (&mut self) {
        
    }

    pub fn display (&self) -> anyhow::Result<()> {
        self.frame.display()
    }
}