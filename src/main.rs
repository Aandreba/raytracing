#![feature(
    portable_simd,
    slice_range,
    ptr_metadata,
    duration_consts_float,
    exit_status_error
)]
use light::{Point, Ambient};

use crate::{
    display::{Camera, Framebuffer},
    element::Element,
    math::Vec3,
    object::sphere::Sphere,
    renderer::Renderer,
};

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )+
    };
}

pub mod display;
pub mod element;
pub mod light;
pub mod math;
pub mod object;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    let frame = Framebuffer::new(100, 100, Camera::default())?; // [120, 50]
    let mut renderer = Renderer::new(
        frame,
        [
            Element::new_unzise(
                Sphere::new(Vec3::new(1.0, -1.0, -1.0), 0.5),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            Element::new_unzise(
                Sphere::new(Vec3::new(1.0, 0.0, -2.0), 0.5),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        ],
        [
            Point::new_unsize(Vec3::ZERO, Vec3::new(1., 1., 1.), 0.5),
            Ambient::new_unsize(0.1 * Vec3::new(1., 1., 1.))
        ],
    );

    renderer.render(1)?;
    Ok(())
}
