#![feature(portable_simd, slice_range, ptr_metadata, exit_status_error)]
use bracket_color::rgb::RGB;
use crate::{display::{Framebuffer, command_prompt_size, Position}, math::{Vec2, Vec3}};

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )+
    };
}

pub mod math;
pub mod object;
pub mod display;
pub mod element;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    println!("Current size: {:?}", command_prompt_size()?);

    let mut frame = Framebuffer::default();
    frame.draw_sphere(Position::Relative(Vec3::new(0.5, 0.5, 2.0)), 3.0, RGB::from_u8(255, 0, 0));
    frame.display()?;

    return Ok(())
}