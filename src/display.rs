use crate::math::{Mat4, Vec3, Vec4, Vec2};
use bracket_color::prelude::HSV;
use rayon::iter::IntoParallelIterator;
use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::io::Write;
use std::ops::{Range, RangeBounds};
use std::simd::f32x4;
use std::str::FromStr;

const ASCII_MAP: &[u8] = b"`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const MAX_INDEX: f32 = (ASCII_MAP.len() - 1) as f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pixels: Box<[u8]>,
    width: usize,
    aspect_ratio: f32,
    pub camera: Camera,
}

impl Camera {
    #[inline]
    pub fn new(fov: f32, z_near: f32, z_far: f32) -> Self {
        debug_assert!(z_near < z_far);
        return Self { fov, z_near, z_far };
    }

    #[inline]
    pub fn transform(self, aspect_ratio: f32) -> Mat4 {
        let yy = f32::tan(self.fov / 2.0).recip();
        let zm = self.z_far - self.z_near;
        let zp = self.z_far + self.z_near;

        return Mat4::from_array([
            [yy / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, yy, 0.0, 0.0],
            [0.0, 0.0, -zp / zm, -(2.0 * self.z_far * self.z_near) / zm],
            [0.0, 0.0, -1.0, 0.0],
        ]);
    }
}

impl Framebuffer {
    #[inline]
    pub fn new(size: Option<[usize; 2]>, camera: Camera) -> Self {
        let [width, height] = size.unwrap_or_else(|| command_prompt_size().unwrap_or_default());
        let [prompt_width, prompt_height] = command_prompt_size().unwrap_or([width, height]);
        let aspect_ratio = (prompt_width as f32) / (prompt_height as f32);

        return Self {
            pixels: vec![ASCII_MAP[0]; width * height].into_boxed_slice(),
            width,
            camera,
            aspect_ratio,
        };
    }

    #[inline]
    pub fn width(&self) -> usize {
        return self.width;
    }

    #[inline]
    pub fn height(&self) -> usize {
        return self.pixels.len() / self.width();
    }

    #[inline]
    pub fn update<H: Into<HSV>, F: Fn(Vec3) -> Option<H>>(
        &mut self,
        f: F,
    ) where
        F: Send + Sync,
        H: Send + Sync,
    {
        self.update_pixel_value(move |x| f(x).map(pixel_value))
    }

    #[inline]
    fn update_pixel_value<F: Fn(Vec3) -> Option<u8>>(
        &mut self,
        f: F,
    ) where
        F: Send + Sync,
    {
        let transform = self.camera.transform(self.aspect_ratio);
        let start_offset = transform * Vec4::new(0.0, 0.0, 1.0, 1.0);

        unsafe {
            self.pixels
                .get_unchecked_mut(..)
                .par_chunks_exact_mut(self.width)
                .enumerate()
                .for_each(|(i, row)| {
                    row.get_unchecked_mut(..)
                        .into_par_iter()
                        .enumerate()
                        .for_each(|(j, x)| {
                            let position = Vec4::new(j as f32, i as f32, 1.0, 1.0);
                            let position = transform * (position + start_offset);
                            if let Some(color) = f(position.into()) {
                                *x = color
                            }
                        })
                });
        }
    }

    #[inline]
    pub fn draw_sphere(&mut self, pos: Position, radius: f32, color: impl Into<HSV>) {
        let pos = pos.to_absolute(self);
        let value = pixel_value(color);

        self.update_pixel_value(
            // f32::round(pos.x() - radius) as usize..=f32::round(pos.x() + radius) as usize,
            // f32::round(pos.y() - radius) as usize..=f32::round(pos.y() + radius) as usize,
            |x| {
                if pos.distance(x) <= radius {
                    return Some(value);
                }
                return None;
            },
        )
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: impl Into<HSV>) {
        self.pixels[x * self.width + y] = pixel_value(color)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.pixels.fill(ASCII_MAP[0])
    }

    #[inline]
    pub fn display(&self) -> anyhow::Result<()> {
        // Clear command prompt
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let mut stdout = std::io::stdout().lock();
        for row in self.pixels.chunks(self.width) {
            stdout.write_all(row)?;
            stdout.write(b"\n")?;
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Absolute (Vec3),
    Relative (Vec3)
}

impl Position {
    #[inline]
    pub fn to_absolute (self, buffer: &Framebuffer) -> Vec3 {
        return match self {
            Self::Absolute(x) => x,
            Self::Relative(x) => {
                const OFFSET: Vec3 = Vec3::splat(1.0);
                let limits = Vec3::new(buffer.width as f32, buffer.height() as f32, 1.0);
                return ((x + OFFSET) / 2.0).wide_mul(limits)
            }
        }
    }

    #[inline]
    pub fn to_relative (self, buffer: &Framebuffer) -> Vec3 {
        return match self {
            Self::Relative(x) => x,
            Self::Absolute(x) => {
                const OFFSET: Vec3 = Vec3::splat(1.0);
                let limits = Vec3::new(buffer.width as f32, buffer.height() as f32, 1.0);
                return 2.0 * x.wide_div(limits) - OFFSET
            }
        }
    }
}

impl Default for Camera {
    #[inline]
    fn default() -> Self {
        Self::new(f32::to_radians(60.0), 0.01, 1000.0)
    }
}

impl Default for Framebuffer {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

#[inline]
pub fn pixel_value(color: impl Into<HSV>) -> u8 {
    unsafe { *ASCII_MAP.get_unchecked((color.into().v * MAX_INDEX) as usize) }
}

#[inline]
pub fn command_prompt_size() -> anyhow::Result<[usize; 2]> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            let width: std::process::Output = powershell_script::run("$Host.UI.RawUI.WindowSize.Width")?.into_inner();
            debug_assert!(width.status.success());
            let width = usize::from_str(std::str::from_utf8(&width.stdout)?.trim())?;

            let height: std::process::Output = powershell_script::run("$Host.UI.RawUI.WindowSize.Height")?.into_inner();
            debug_assert!(height.status.success());
            let height = usize::from_str(std::str::from_utf8(&height.stdout)?.trim())?;

            return Ok([width, height])
        } else {
            todo!()
        }
    }
}
