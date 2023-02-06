use std::{simd::{SimdFloat, f32x2, mask32x2, simd_swizzle}, ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg}, fmt::Debug};
use super::{Vec3, Vec4};

pub type Mask2 = mask32x2;

/// A vector with to lanes with a known norm of one.
#[derive(Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct UnitVec2 (Vec2);

#[derive(Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct Vec2 (f32x2);

impl Vec2 {
    pub const ZERO: Self = Self::splat_const(0.0);

    #[inline]
    pub const fn new (x: f32, y: f32) -> Self {
        return Self(f32x2::from_array([x, y]))
    }

    #[inline]
    pub fn splat (v: f32) -> Self {
        return Self(f32x2::splat(v))
    }

    #[inline]
    pub const fn splat_const (v: f32) -> Self {
        return Self::new(v, v)
    }

    #[inline]
    pub const fn from_simd (v: f32x2) -> Self {
        return Self(v)
    }

    #[inline]
    pub const fn to_inner (self) -> f32x2 {
        self.0
    }

    #[inline]
    pub fn wide_mul (self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }

    #[inline]
    pub fn wide_div (self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    #[inline]
    pub fn reduce_add (self) -> f32 {
        return self.0.reduce_sum()
    }

    #[inline]
    pub fn dot (self, rhs: Vec2) -> f32 {
        Self(self.0 * rhs.0).reduce_add()
    }

    #[inline]
    pub fn sq_norm (self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn norm (self) -> f32 {
        f32::hypot(self.x(), self.y())
    }

    #[inline]
    pub fn unit (self) -> UnitVec2 {
        UnitVec2(self / self.norm())
    }

    #[inline]
    pub fn distance (self, rhs: Vec2) -> f32 {
        (self - rhs).norm()
    }

    #[inline]
    pub fn angle (self, rhs: Vec2) -> f32 {
        f32::acos((self * rhs) / (self.norm() * rhs.norm()))
    }

    #[inline]
    pub fn x (self) -> f32 {
        return self.0[0]
    }

    #[inline]
    pub fn y (self) -> f32 {
        return self.0[1]
    }

    #[inline]
    pub fn set_x (&mut self, v: f32) {
        self.0[0] = v
    }

    #[inline]
    pub fn set_y (&mut self, v: f32) {
        self.0[1] = v
    }

    #[inline]
    pub fn as_array (&self) -> &[f32; 2] {
        self.0.as_array()
    }
    
    #[inline]
    pub fn into_array (self) -> [f32; 2] {
        *self.as_array()
    }
}

impl UnitVec2 {
    #[inline]
    pub fn new (x: f32, y: f32) -> Option<Self> {
        Self::from_vec(Vec2::new(x, y))
    }

    #[inline]
    pub unsafe fn new_unchecked (x: f32, y: f32) -> Self {
        Self::from_vec_unchecked(Vec2::new(x, y))
    }

    #[inline]
    pub fn from_vec (vec: Vec2) -> Option<Self> {
        if f32::abs(vec.sq_norm() - 1.) <= f32::EPSILON { return Some(Self(vec)) }
        return None
    }
    
    #[inline]
    pub unsafe fn from_vec_unchecked (vec: Vec2) -> Self {
        debug_assert!(f32::abs(vec.sq_norm() - 1.) <= f32::EPSILON);
        Self(vec)
    }

    #[inline]
    pub fn from_simd (v: f32x2) -> Option<Self> {
        return Self::from_vec(Vec2::from_simd(v))
    }

    #[inline]
    pub unsafe fn from_simd_unchecked (v: f32x2) -> Self {
        return Self::from_vec_unchecked(Vec2::from_simd(v))
    }

    #[inline]
    pub const fn to_vec (self) -> Vec2 {
        self.0
    }

    #[inline]
    pub const fn to_inner (self) -> f32x2 {
        self.0.to_inner()
    }

    // #[inline]
    // pub fn wide_mul (self, rhs: Self) -> Self {
    //     Self(self.0 * rhs.0)
    // }
    // #[inline]
    // pub fn wide_div (self, rhs: Self) -> Self {
    //     Self(self.0 / rhs.0)
    // }

    #[inline]
    pub fn reduce_add (self) -> f32 {
        return self.0.reduce_add()
    }

    #[inline]
    pub fn dot_vec (self, rhs: Vec2) -> f32 {
        self.0.dot(rhs)
    }

    #[inline]
    pub fn dot (self, rhs: Self) -> f32 {
        self.dot_vec(rhs.0)
    }

    #[inline]
    pub fn distance (self, rhs: Vec2) -> f32 {
        (self - rhs).norm()
    }

    #[inline]
    pub fn angle (self, rhs: Self) -> f32 {
        f32::acos(self.dot(rhs))
    }

    #[inline]
    pub fn x (self) -> f32 {
        return self.0.x()
    }

    #[inline]
    pub fn y (self) -> f32 {
        return self.0.y()
    }

    #[inline]
    pub fn as_array (&self) -> &[f32; 2] {
        self.0.as_array()
    }
    
    #[inline]
    pub fn into_array (self) -> [f32; 2] {
        *self.as_array()
    }
}

impl Vec2 {
    #[inline]
    pub fn is_finite (self) -> bool {
        self.is_finite_mask().all()
    }

    #[inline]
    pub fn is_infinite (self) -> bool {
        self.is_infinite_mask().any()
    }

    #[inline]
    pub fn is_nan (self) -> bool {
        self.is_nan_mask().any()
    }

    #[inline]
    pub fn is_normal (self) -> bool {
        self.is_normal_mask().all()
    }

    #[inline]
    pub fn is_finite_mask (self) -> Mask2 {
        self.0.is_finite()
    }
    
    #[inline]
    pub fn is_infinite_mask (self) -> Mask2 {
        self.0.is_infinite()
    }
    
    #[inline]
    pub fn is_nan_mask (self) -> Mask2 {
        self.0.is_nan()
    }

    #[inline]
    pub fn is_normal_mask (self) -> Mask2 {
        self.0.is_normal()
    }

    #[inline]
    pub fn is_sign_positive_mask (self) -> Mask2 {
        self.0.is_sign_positive()
    }

    #[inline]
    pub fn is_sign_negative_mask (self) -> Mask2 {
        self.0.is_sign_negative()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Vec2 {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * f32x2::splat(rhs))
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(f32x2::splat(self) * rhs.0)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= f32x2::splat(rhs)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / f32x2::splat(rhs))
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2(f32x2::splat(self) / rhs.0)
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= f32x2::splat(rhs)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

/* UNIT VEC */
impl Add for UnitVec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Add<Vec2> for UnitVec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        self.0 + rhs
    }
}

impl Add<UnitVec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: UnitVec2) -> Self::Output {
        self + rhs.0
    }
}

impl Sub for UnitVec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<Vec2> for UnitVec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        self.0 - rhs
    }
}

impl Sub<UnitVec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: UnitVec2) -> Self::Output {
        self - rhs.0
    }
}

impl Mul for UnitVec2 {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<Vec2> for UnitVec2 {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.dot_vec(rhs)
    }
}

impl Mul<f32> for UnitVec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<UnitVec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: UnitVec2) -> Self::Output {
        self * rhs.0
    }
}

impl Div<f32> for UnitVec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0 / rhs
    }
}

impl Div<UnitVec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: UnitVec2) -> Self::Output {
        self / rhs.0
    }
}

impl Neg for UnitVec2 {
    type Output = UnitVec2;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

/* CONVERSIONS */
impl From<Vec3> for Vec2 {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self::from_simd(simd_swizzle!(value.to_inner(), [0, 1]))
    }
}

impl From<Vec4> for Vec2 {
    #[inline]
    fn from(value: Vec4) -> Self {
        Self::from_simd(simd_swizzle!(value.to_inner(), [0, 1]))
    }
}

impl Debug for Vec2 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}