use bracket_color::prelude::HSV;
use bracket_color::rgb::RGB;
use rayon::iter::IntoParallelIterator;
use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::io::Write;
use std::ops::{Range, RangeBounds};
use std::str::FromStr;

use crate::vec2::Vec2;

const ASCII_MAP: &[u8] = b"`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const MAX_INDEX: f32 = (ASCII_MAP.len() - 1) as f32;

pub struct Framebuffer {
    pixels: Box<[u8]>,
    width: usize,
}

impl Framebuffer {
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            pixels: vec![ASCII_MAP[0]; width * height].into_boxed_slice(),
            width,
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
    pub fn update<H: Into<HSV>, F: Fn(usize, usize) -> Option<H>>(
        &mut self,
        region_x: impl RangeBounds<usize>,
        region_y: impl RangeBounds<usize>,
        f: F,
    ) where
        F: Send + Sync,
        H: Send + Sync,
    {
        self.update_pixel_value(region_x, region_y, move |i, j| f(i, j).map(pixel_value))
    }

    #[inline]
    fn update_pixel_value<F: Fn(usize, usize) -> Option<u8>>(
        &mut self,
        region_x: impl RangeBounds<usize>,
        region_y: impl RangeBounds<usize>,
        f: F,
    ) where
        F: Send + Sync,
    {

        pub fn clamped_range<R: RangeBounds<usize>>(range: R, bounds: std::ops::RangeTo<usize>) -> std::ops::Range<usize> {
            let len = bounds.end;

            let start: std::ops::Bound<&usize> = range.start_bound();
            let start = match start {
                std::ops::Bound::Included(&start) => start,
                std::ops::Bound::Excluded(start) => start + 1,
                std::ops::Bound::Unbounded => 0,
            };
        
            let end: std::ops::Bound<&usize> = range.end_bound();
            let end = match end {
                std::ops::Bound::Included(end) => end + 1,
                std::ops::Bound::Excluded(&end) => end,
                std::ops::Bound::Unbounded => len,
            };
        
            std::ops::Range {
                start: usize::min(start, len),
                end: usize::min(end, len)
            }
        }

        let Range { start: row_start, end: row_end } = clamped_range(region_x, ..self.pixels.len());
        let Range { start: col_start, end: col_end } = clamped_range(region_y, ..self.width);
        
        unsafe {
            self.pixels
                .get_unchecked_mut(row_start..row_end)
                .par_chunks_exact_mut(row_end - row_start)
                .enumerate()
                .for_each(|(i, row)| {
                    row.get_unchecked_mut(col_start..col_end)
                        .into_par_iter()
                        .enumerate()
                        .for_each(|(j, x)| {
                            if let Some(color) = f(i + row_start, j + col_start) {
                                *x = color
                            }
                        })
                });
        }
    }

    #[inline]
    pub fn draw_circle (&mut self, pos: Vec2, radius: f32, color: impl Into<HSV>) {
        let value = pixel_value(color);
        self.update_pixel_value(
            f32::round(pos.x() - radius) as usize..=f32::round(pos.x() + radius) as usize,
            f32::round(pos.y() - radius) as usize..=f32::round(pos.y() + radius) as usize,
            |x, y| {
                if Vec2::new(x as f32, y as f32).distance(pos) <= radius { return Some(value) }
                return None
            }
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

impl Default for Framebuffer {
    #[inline]
    fn default() -> Self {
        let [width, height] = command_prompt_size().unwrap_or_default();
        return Self::new(width, height);
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
