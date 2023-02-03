#![feature(portable_simd, slice_range, ptr_metadata, exit_status_error)]
use bracket_color::rgb::RGB;
use crate::{display::{Framebuffer, command_prompt_size}, vec2::Vec2};

pub mod vec2;
pub mod vec3;

pub mod object;
pub mod display;
pub mod element;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    println!("Current size: {:?}", command_prompt_size()?);

    let mut frame = Framebuffer::new(10, 10);
    frame.draw_circle(Vec2::new(0.0, 2.0), 2.5, RGB::from_u8(255, 0, 0)); // todo fix
    frame.display()?;

    return Ok(())
}