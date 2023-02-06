use super::{Vec3, Vec4, EulerAngles};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub, AddAssign, SubAssign, MulAssign, DivAssign},
    simd::{f32x4, simd_swizzle, Which},
};

/// Quaternion that is known to have a norm of one
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Versor(Quaternion);

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

    #[inline]
    pub fn from_euler (euler: EulerAngles) -> Self {
        let euler = euler.to_radians();
        let half = 0.5 * euler;

        let sin = Vec3::from_array(half.to_array().map(f32::sin)).to_inner();
        let cos = Vec3::from_array(half.to_array().map(f32::cos)).to_inner();

        let alpha_1 = simd_swizzle!(sin, cos, [Which::Second(0), Which::First(0), Which::Second(0), Which::Second(0)]);
        let alpha_2 = simd_swizzle!(sin, cos, [Which::Second(1), Which::Second(1), Which::First(1), Which::Second(1)]);
        let alpha_3 = simd_swizzle!(sin, cos, [Which::Second(2), Which::Second(2), Which::Second(2), Which::First(2)]);
        let alpha = alpha_1 * alpha_2 * alpha_3;

        let beta_1 = simd_swizzle!(sin, cos, [Which::First(0), Which::Second(0), Which::First(0), Which::First(0)]);
        let beta_2 = simd_swizzle!(sin, cos, [Which::First(1), Which::First(1), Which::Second(1), Which::First(1)]);
        let beta_3 = simd_swizzle!(sin, cos, [Which::First(2), Which::First(2), Which::First(2), Which::Second(2)]);
        let beta = beta_1 * beta_2 * beta_3 * f32x4::from_array([1., -1., 1., -1.]);

        return Quaternion::from_simd(alpha + beta)
    }

    #[inline]
    pub const fn to_inner (self) -> f32x4 {
        return self.0.to_inner()
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

impl Versor {
    #[inline]
    pub fn new (q: Quaternion) -> Option<Self> {
        if q.sq_norm() == 1. {
            return Some(Self(q))
        }
        return None
    }

    #[inline]
    pub unsafe fn new_unchecked (q: Quaternion) -> Self {
        debug_assert_eq!(q.sq_norm(), 1.);
        return Self(q)
    }
    
    #[inline]
    pub fn from_quaternion (q: Quaternion) -> Self {
        unsafe { Self::new_unchecked(q.unit()) }
    }

    #[inline]
    pub fn from_euler (euler: EulerAngles) -> Self {
        Self(Quaternion::from_euler(euler))
    }

    #[inline]
    pub const fn to_inner (self) -> Quaternion {
        self.0
    }
}

impl Versor {
    #[inline]
    pub fn conjugate(self) -> Versor {
        return Self(Quaternion(self.0.0.wide_mul(Vec4::from_array([1., -1., -1., -1.]))));
    }

    #[inline]
    pub fn inverse(self) -> Versor {
        return self.conjugate()
    }
}

impl Versor {
    // perhaps optimizable
    // https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Quaternion-derived_rotation_matrix
    #[inline]
    pub fn apply(self, v: Vec3) -> Vec3 {
        let p = Quaternion(Vec4::from_simd(simd_swizzle!(v.to_inner(), [3, 0, 1, 2])));
        let inv = self.inverse();
        let res = (self * p) * inv;
        
        debug_assert!(res.r() <= f32::EPSILON);
        unsafe { Vec3::from_simd_unchecked(simd_swizzle!(res.to_inner(), [1, 2, 3, 0])) }
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Quaternion {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Quaternion {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.0.to_inner();
        let b = rhs.0.to_inner();

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

impl MulAssign for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion(self.0 * rhs)
    }
}

impl MulAssign<f32> for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
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
        Quaternion(self.0 / rhs)
    }
}

impl DivAssign<f32> for Quaternion {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs
    }
}

impl Div<Quaternion> for f32 {
    type Output = Quaternion;

    #[inline]
    fn div(self, rhs: Quaternion) -> Self::Output {
        Quaternion(Vec4::splat(self).wide_div(rhs.0))
    }
}

impl Add for Versor {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Add<Quaternion> for Versor {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: Quaternion) -> Self::Output {
        self.0 + rhs
    }
}

impl Add<Versor> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: Versor) -> Self::Output {
        self + rhs.0
    }
}

impl Sub for Versor {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<Quaternion> for Versor {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: Quaternion) -> Self::Output {
        self.0 - rhs
    }
}

impl Sub<Versor> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: Versor) -> Self::Output {
        self - rhs.0
    }
}

impl Mul for Versor {
    type Output = Versor;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Versor {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Mul<Quaternion> for Versor {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Quaternion) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<Versor> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Versor) -> Self::Output {
        self * rhs.0
    }
}

impl Mul<f32> for Versor {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<Versor> for f32 {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Versor) -> Self::Output {
        self * rhs.0
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

impl Debug for Versor {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Default for Versor {
    #[inline]
    fn default() -> Self {
        Versor(Quaternion::new(1., 0., 0., 0.))
    }
}

impl From<EulerAngles> for Quaternion {
    #[inline]
    fn from(value: EulerAngles) -> Self {
        Quaternion::from_euler(value)
    }
}

impl From<EulerAngles> for Versor {
    #[inline]
    fn from(value: EulerAngles) -> Self {
        Versor::from_euler(value)
    }
}

impl From<Quaternion> for Versor {
    #[inline]
    fn from(value: Quaternion) -> Self {
        Versor::from_quaternion(value)
    }
}

impl From<Versor> for Quaternion {
    #[inline]
    fn from(value: Versor) -> Self {
        value.to_inner()
    }
}