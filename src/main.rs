#![feature(
    portable_simd,
    slice_range,
    ptr_metadata,
    duration_consts_float,
    exit_status_error
)]
use crate::{
    display::{Camera, Framebuffer},
    element::Element,
    math::Vec3,
    object::sphere::Sphere,
    renderer::Renderer
};
use std::{
    cell::UnsafeCell,
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
pub mod math;
pub mod object;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    let frame = Framebuffer::new(100, 100, Camera::default())?; // [120, 50]
    let mut renderer = Renderer::new(frame, Vec::new());
    let _ = renderer.push(Element::new_unzise(
        Sphere::new(Vec3::new(0.0, 0.0, -2.0), 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    ));

    renderer.render(1)?;
    Ok(())
}

#[allow(unused)]
#[inline]
fn wait_until_press() -> std::io::Result<()> {
    thread_local! {
        static GB: UnsafeCell<String> = UnsafeCell::new(String::new());
    }

    GB.with(|gb| unsafe {
        let gb = &mut *gb.get();
        gb.clear();
        std::io::stdin().read_line(gb)
    })?;

    return Ok(());
}
