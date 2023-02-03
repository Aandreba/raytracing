#![feature(
    portable_simd,
    slice_range,
    ptr_metadata,
    duration_consts_float,
    exit_status_error
)]
use std::{
    cell::UnsafeCell,
    time::{Duration, Instant},
};
use crate::{
    display::{Camera, Framebuffer, Position},
    math::Vec3,
    renderer::Renderer, object::sphere::Sphere, element::Element,
};
use bracket_color::rgb::RGB;

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
    const BUDGET: Duration = Duration::from_secs_f64(1.0 / 24.0);

    let frame = Framebuffer::new(None, Camera::default()); // [120, 50]
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 5.0);
    let mut renderer = Renderer::new(frame, vec![Element::new_unzise(&sphere, RGB::from_f32(1.0, 0.0, 0.0))]);

    loop {
        let start = Instant::now();
        //position.set_x(f32::clamp(position.x() + BUDGET.as_secs_f32(), -1.0, 1.0));
        renderer.render(1)?;
        break;

        if let Some(delta) = BUDGET.checked_sub(start.elapsed()) {
            std::thread::sleep(delta)
        }
    }

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
