use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    simd::{f32x4, mask32x4, simd_swizzle, SimdFloat, Which},
};

use super::{Vec2, Vec3};

pub type Mask4 = mask32x4;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct Vec4(f32x4);

impl Vec4 {
    pub const ZERO: Self = Self::splat_const(0.0);

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self::from_array([x, y, z, w]);
    }

    #[inline]
    pub const fn from_array(v: [f32; 4]) -> Self {
        return Self(f32x4::from_array(v));
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        return Self(f32x4::splat(v));
    }

    #[inline]
    pub const fn splat_const(v: f32) -> Self {
        return Self::from_array([v; 4]);
    }

    #[inline]
    pub const fn from_simd(v: f32x4) -> Self {
        return Self(v);
    }

    pub fn from_vec2(xy: Vec2, zw: Vec2) -> Self {
        return Self::from_simd(simd_swizzle!(
            xy.into_inner(),
            zw.into_inner(),
            [
                Which::First(0),
                Which::First(1),
                Which::Second(0),
                Which::Second(1)
            ]
        ));
    }

    #[inline]
    pub fn from_vec3(xyz: Vec3, w: f32) -> Self {
        let mut this = xyz.into_inner();
        this[2] = w;
        return Self::from_simd(this);
    }

    #[inline]
    pub const fn into_inner(self) -> f32x4 {
        self.0
    }
}

impl Vec4 {
    #[inline]
    pub fn wide_mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }

    #[inline]
    pub fn wide_div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    #[inline]
    pub fn reduce_add(self) -> f32 {
        cfg_if::cfg_if! {
            if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))] {
                #[cfg(target_arch = "x86")]
                use std::arch::x86::*;
                #[cfg(target_arch = "x86_64")]
                use std::arch::x86_64::*;

                #[inline]
                #[allow(non_snake_case)]
                const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
                    ((z << 6) | (y << 4) | (x << 2) | w) as i32
                }

                let v: __m128 = self.0.into();
                unsafe {
                    // [ C D | A B ]
                    #[cfg(target_feature = "sse3")]
                    let shuf = _mm_movehdup_ps(v);
                    #[cfg(not(target_feature = "sse3"))]
                    let shuf = _mm_shuffle_ps(v, v, _MM_SHUFFLE(2, 3, 0, 1));
                    // sums = [ D+C C+D | B+A A+B ]
                    let sums = _mm_add_ps(v, shuf);
                    //  [   C   D | D+C C+D ]  // let the compiler avoid a mov by reusing shuf
                    let shuf = _mm_movehl_ps(shuf, sums);
                    let sums = _mm_add_ss(sums, shuf);
                    return _mm_cvtss_f32(sums);
                }
            } else {
                return self.0.reduce_sum()
            }
        }
    }

    #[inline]
    pub fn dot(self, rhs: Vec4) -> f32 {
        Self(self.0 * rhs.0).reduce_add()
    }

    #[inline]
    pub fn sq_norm(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn norm(self) -> f32 {
        self.sq_norm().sqrt()
    }

    #[inline]
    pub fn unit(self) -> Vec4 {
        self / self.norm()
    }

    #[inline]
    pub fn distance(self, rhs: Vec4) -> f32 {
        (self - rhs).norm()
    }

    #[inline]
    pub fn x(&self) -> f32 {
        return self.0[0];
    }

    #[inline]
    pub fn y(&self) -> f32 {
        return self.0[1];
    }

    #[inline]
    pub fn as_array(&self) -> &[f32; 4] {
        self.0.as_array()
    }

    #[inline]
    pub fn into_array(self) -> [f32; 4] {
        *self.as_array()
    }
}

impl Vec4 {
    #[inline]
    pub fn is_finite(self) -> bool {
        self.is_finite_mask().all()
    }

    #[inline]
    pub fn is_infinite(self) -> bool {
        self.is_infinite_mask().any()
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.is_nan_mask().any()
    }

    #[inline]
    pub fn is_normal(self) -> bool {
        self.is_normal_mask().all()
    }

    #[inline]
    pub fn is_finite_mask(self) -> Mask4 {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_infinite_mask(self) -> Mask4 {
        self.0.is_infinite()
    }

    #[inline]
    pub fn is_nan_mask(self) -> Mask4 {
        self.0.is_nan()
    }

    #[inline]
    pub fn is_normal_mask(self) -> Mask4 {
        self.0.is_normal()
    }

    #[inline]
    pub fn is_sign_positive_mask(self) -> Mask4 {
        self.0.is_sign_positive()
    }

    #[inline]
    pub fn is_sign_negative_mask(self) -> Mask4 {
        self.0.is_sign_negative()
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Vec4 {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * f32x4::splat(rhs))
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4(f32x4::splat(self) * rhs.0)
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= f32x4::splat(rhs)
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / f32x4::splat(rhs))
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;

    #[inline]
    fn div(self, rhs: Vec4) -> Self::Output {
        Vec4(f32x4::splat(self) / rhs.0)
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= f32x4::splat(rhs)
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}