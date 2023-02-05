use crate::math::{Mat4, Vec3, Vec4};
use image::{ImageBuffer, Rgb};
use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[derive(Debug)]
pub struct Framebuffer {
    pixels: ImageBuffer<Rgb<f32>, Box<[f32]>>,
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
            [0.0, 0.0, -zp / zm, (-2.0 * self.z_far * self.z_near) / zm],
            [0.0, 0.0, -1.0, 0.0],
        ]);
    }
}

impl Framebuffer {
    #[inline]
    pub fn new(width: u32, height: u32, camera: Camera) -> anyhow::Result<Self> {
        let aspect_ratio = (width as f32) / (height as f32);

        return Ok(Self {
            pixels: unsafe {
                ImageBuffer::from_raw(
                    width,
                    height,
                    vec![0.0; 3 * (width as usize) * (height as usize)].into_boxed_slice(),
                )
                .unwrap_unchecked()
            },
            camera,
            aspect_ratio,
        });
    }

    #[inline]
    pub fn width(&self) -> u32 {
        return self.pixels.width();
    }

    #[inline]
    pub fn height(&self) -> u32 {
        return self.pixels.height();
    }

    #[inline]
    pub fn update<T, I: FnOnce(Mat4) -> T, F: Fn(Vec3, &T) -> Option<Rgb<f32>>>(
        &mut self,
        init: I,
        f: F,
    ) where
        T: Sync,
        F: Send + Sync,
    {
        let transform = self.camera.transform(self.aspect_ratio);
        let t = &init(transform);

        let width = self.width();
        let size = Vec4::new(width as f32, self.height() as f32, 1.0, 1.0);

        unsafe {
            let pixels = core::slice::from_raw_parts_mut(self.pixels.as_mut_ptr().cast::<Rgb<f32>>(), (self.width() as usize) * (self.height() as usize));
            pixels
                .par_chunks_exact_mut(width as usize)
                .enumerate()
                .for_each(|(i, row)| {
                    row.get_unchecked_mut(..)
                        .into_iter()
                        .enumerate()
                        .for_each(|(j, x)| {
                            let position = transform
                                * (2. * Vec4::new(j as f32, i as f32, 1.0, 1.0).wide_div(size)
                                    - Vec4::splat(1.0));
                            if let Some(color) = f(position.into(), t) {
                                *x = color
                            }
                        })
                });
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.pixels.fill(0.0)
    }

    #[inline]
    pub fn display(&self) -> anyhow::Result<()> {
        self.pixels.save_with_format("result.png", image::ImageFormat::Png).map_err(Into::into)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Absolute(Vec3),
    Relative(Vec3),
}

impl Position {
    #[inline]
    pub fn to_absolute(self, buffer: &Framebuffer) -> Vec3 {
        return match self {
            Self::Absolute(x) => x,
            Self::Relative(x) => {
                const OFFSET: Vec3 = Vec3::new(1.0, 1.0, 0.0);
                let limits = Vec3::new(buffer.width() as f32, buffer.height() as f32, 1.0);
                return ((x + OFFSET) / 2.0).wide_mul(limits);
            }
        };
    }

    #[inline]
    pub fn to_relative(self, buffer: &Framebuffer) -> Vec3 {
        return match self {
            Self::Relative(x) => x,
            Self::Absolute(x) => {
                const OFFSET: Vec3 = Vec3::new(1.0, 1.0, 0.0);
                let limits = Vec3::new(buffer.width() as f32, buffer.height() as f32, 1.0);
                return 2.0 * x.wide_div(limits) - OFFSET;
            }
        };
    }
}

impl Default for Camera {
    #[inline]
    fn default() -> Self {
        Self::new(f32::to_radians(60.0), 0.01, 1000.0)
    }
}
