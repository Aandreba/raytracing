use super::{Vec4, Vec3};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
    simd::{f32x4, simd_swizzle},
};

#[derive(Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct Quaternion(Vec4);

impl Quaternion {
    #[inline]
    pub const fn new(r: f32, i: f32, j: f32, k: f32) -> Self {
        Self::from_array([r, i, j, k])
    }

    #[inline]
    pub const fn from_array(v: [f32; 4]) -> Self {
        Self::from_simd(f32x4::from_array(v))
    }

    #[inline]
    pub const fn from_vec(v: Vec4) -> Self {
        return Self(v);
    }

    #[inline]
    pub const fn from_simd(v: f32x4) -> Self {
        return Self(Vec4::from_simd(v));
    }

    pub fn from_euler () -> Self {
        todo!()
    }

    #[inline]
    pub fn r(self) -> f32 {
        return self.0.x();
    }

    #[inline]
    pub fn i(self) -> f32 {
        return self.0.y();
    }

    #[inline]
    pub fn j(self) -> f32 {
        return self.0.z();
    }

    #[inline]
    pub fn k(self) -> f32 {
        return self.0.w();
    }
}

impl Quaternion {
    #[inline]
    pub fn sq_norm(self) -> f32 {
        self.0.sq_norm()
    }

    #[inline]
    pub fn norm(self) -> f32 {
        self.0.norm()
    }

    #[inline]
    pub fn unit(self) -> Quaternion {
        self / self.norm()
    }

    #[inline]
    pub fn conjugate(self) -> Quaternion {
        return Self(self.0.wide_mul(Vec4::from_array([1., -1., -1., -1.])));
    }

    #[inline]
    pub fn inverse(self) -> Quaternion {
        return self.conjugate() / self.sq_norm();
    }
}

impl Quaternion {
    #[inline]
    pub fn apply (self, v: Vec3) -> Vec3 {
        todo!()
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.0.into_inner();
        let b = rhs.0.into_inner();

        let a1123 = simd_swizzle!(a, [1, 1, 2, 3]);
        let a2231 = simd_swizzle!(a, [2, 2, 3, 1]);
        let b1000 = simd_swizzle!(b, [1, 0, 0, 0]);
        let b2312 = simd_swizzle!(b, [2, 3, 1, 2]);
        let t1 = a1123 * b1000;
        let t2 = a2231 * b2312;
        let t12 = t1 + t2;
        let t12m = t12 * f32x4::from_array([-1., 1., 1., 1.]);
        let a3312 = simd_swizzle!(a, [3, 3, 1, 2]);
        let b3231 = simd_swizzle!(b, [3, 2, 3, 1]);
        let a0000 = simd_swizzle!(a, [0, 0, 0, 0]);
        let t3 = a3312 * b3231;
        let t0 = a0000 * b;
        let t03 = t0 - t3;

        return Self::from_simd(t03 + t12m);
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion(self.0.wide_mul(Vec4::splat(rhs)))
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion(Vec4::splat(self).wide_mul(rhs.0))
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Quaternion(self.0.wide_div(Vec4::splat(rhs)))
    }
}

impl Div<Quaternion> for f32 {
    type Output = Quaternion;

    #[inline]
    fn div(self, rhs: Quaternion) -> Self::Output {
        Quaternion(Vec4::splat(self).wide_div(rhs.0))
    }
}

impl Debug for Quaternion {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} + {}i + {}j + {}j",
            self.r(),
            self.i(),
            self.j(),
            self.k()
        )
    }
}
