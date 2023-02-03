use super::Vec4;
use std::simd::{f32x4, Which};
use std::{ops::*, simd::simd_swizzle};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4::from_diagonal_splat(1.0);

    #[inline]
    pub const fn from_rows (x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        return Self {
            x,
            y,
            z,
            w,
        }
    }

    #[inline]
    pub const fn from_array(v: [[f32; 4]; 4]) -> Self {
        return Self {
            x: Vec4::from_array(v[0]),
            y: Vec4::from_array(v[1]),
            z: Vec4::from_array(v[2]),
            w: Vec4::from_array(v[3]),
        };
    }

    #[inline]
    pub const fn from_flat_array(v: [f32; 16]) -> Self {
        unsafe { return Self::from_array(core::mem::transmute(v)) }
    }

    #[inline]
    pub const fn from_diagonal(v: [f32; 4]) -> Self {
        return Self::from_array([
            [v[0], 0.0, 0.0, 0.0],
            [0.0, v[1], 0.0, 0.0],
            [0.0, 0.0, v[2], 0.0],
            [0.0, 0.0, 0.0, v[3]],
        ]);
    }

    #[inline]
    pub const fn from_diagonal_splat(v: f32) -> Self {
        return Self::from_diagonal([v; 4]);
    }
}

impl Mat4 {
    #[inline]
    pub fn transpose(mut self) -> Self {
        self.transpose_assign();
        self
    }

    #[inline]
    pub fn transpose_assign(&mut self) {
        #[inline]
        fn unpacklo(lhs: f32x4, rhs: f32x4) -> f32x4 {
            simd_swizzle!(
                lhs,
                rhs,
                [
                    Which::First(0),
                    Which::Second(0),
                    Which::First(1),
                    Which::Second(1)
                ]
            )
        }

        #[inline]
        fn unpackhi(lhs: f32x4, rhs: f32x4) -> f32x4 {
            simd_swizzle!(
                lhs,
                rhs,
                [
                    Which::First(2),
                    Which::Second(2),
                    Which::First(3),
                    Which::Second(3)
                ]
            )
        }

        #[inline]
        fn movelh(lhs: f32x4, rhs: f32x4) -> f32x4 {
            simd_swizzle!(
                lhs,
                rhs,
                [
                    Which::First(0),
                    Which::First(1),
                    Which::Second(0),
                    Which::Second(1)
                ]
            )
        }

        #[inline]
        fn movehl(lhs: f32x4, rhs: f32x4) -> f32x4 {
            simd_swizzle!(
                lhs,
                rhs,
                [
                    Which::Second(2),
                    Which::Second(3),
                    Which::First(2),
                    Which::First(3)
                ]
            )
        }

        let tmp0 = unpacklo(self.x.into_inner(), self.y.into_inner());
        let tmp1 = unpacklo(self.z.into_inner(), self.w.into_inner());
        let tmp2 = unpackhi(self.x.into_inner(), self.y.into_inner());
        let tmp3 = unpackhi(self.z.into_inner(), self.w.into_inner());

        self.x = Vec4::from_simd(movelh(tmp0, tmp1));
        self.y = Vec4::from_simd(movehl(tmp2, tmp0));
        self.z = Vec4::from_simd(movelh(tmp1, tmp3));
        self.w = Vec4::from_simd(movehl(tmp3, tmp1));
    }
}

impl Add for Mat4 {
    type Output = Mat4;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        };
    }
}

impl Sub for Mat4 {
    type Output = Mat4;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        };
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    #[inline]
    fn mul(self, mut rhs: Self) -> Self::Output {
        rhs.transpose_assign();
        return Self::from_rows(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
            self * rhs.w
        )
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        return Vec4::from_array([
            self.x.dot(rhs),
            self.y.dot(rhs),
            self.z.dot(rhs),
            self.w.dot(rhs)
        ])
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        return Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        };
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;

    #[inline]
    fn mul(self, rhs: Mat4) -> Self::Output {
        return Mat4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Div<f32> for Mat4 {
    type Output = Mat4;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        return Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        };
    }
}

impl Div<Mat4> for f32 {
    type Output = Mat4;

    #[inline]
    fn div(self, rhs: Mat4) -> Self::Output {
        return Mat4 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
            w: self / rhs.w,
        }
    }
}