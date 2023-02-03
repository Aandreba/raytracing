use std::{simd::{f32x4, mask32x4, i32x4, SimdFloat, simd_swizzle}, ops::{Add, Sub, Mul, Div, BitOr, BitAnd, BitXor, Not, AddAssign, SubAssign, MulAssign, DivAssign, Neg}};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct Mask3 (mask32x4);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct Vec3 (f32x4);

impl Vec3 {
    pub const ZERO: Self = Self::splat(0.0);

    #[inline]
    pub const fn new (x: f32, y: f32, z: f32) -> Self {
        return Self(f32x4::from_array([x, y, z, 0.0]))
    }

    #[inline]
    pub const fn splat (v: f32) -> Self {
        return Self::new(v, v, v)
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
    pub fn dot (self, rhs: Vec3) -> f32 {
        Self(self.0 * rhs.0).reduce_add()
    }

    #[inline]
    pub fn cross (self, rhs: Vec3) -> Vec3 {
        // x  <-  a.y*b.z - a.z*b.y
        // y  <-  a.z*b.x - a.x*b.z
        // z  <-  a.x*b.y - a.y*b.x
        // We can save a shuffle by grouping it in this wacky order:
        // (self.zxy() * rhs - self * rhs.zxy()).zxy()
        //     let lhszxy = _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10);
        //     let rhszxy = _mm_shuffle_ps(rhs.0, rhs.0, 0b01_01_00_10);
        //     let lhszxy_rhs = _mm_mul_ps(lhszxy, rhs.0);
        //     let rhszxy_lhs = _mm_mul_ps(rhszxy, self.0);
        //     let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
        //     Self(_mm_shuffle_ps(sub, sub, 0b01_01_00_10))

        let lhszxy = simd_swizzle!(self.0, [1, 1, 0, 0]);
        let rhszxy = simd_swizzle!(self.0, [1, 1, 0, 2]);
        let lhszxy_rhs = lhszxy * rhs.0;
        let rhszxy_lhs = rhszxy * self.0;
        let sub = lhszxy_rhs - rhszxy_lhs;
        return Self(simd_swizzle!(sub, [1, 1, 0, 2]))
    }

    #[inline]
    pub fn sq_norm (self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn norm (self) -> f32 {
        self.sq_norm().sqrt()
    }

    #[inline]
    pub fn unit (self) -> Vec3 {
        self / self.norm()
    }

    #[inline]
    pub fn distance (self, rhs: Vec3) -> f32 {
        (self - rhs).norm()
    }

    #[inline]
    pub fn x (&self) -> f32 {
        return self.0[0]
    }

    #[inline]
    pub fn y (&self) -> f32 {
        return self.0[1]
    }

    #[inline]
    pub fn z (&self) -> f32 {
        return self.0[2]
    }

    #[inline]
    pub fn as_array (&self) -> &[f32; 3] {
        unsafe { &*(self.0.as_array() as *const [f32; 4] as *const [f32; 3]) }
    }
    
    #[inline]
    pub fn into_array (self) -> [f32; 3] {
        unsafe { *(self.0.as_array() as *const [f32; 4] as *const [f32; 3]) }
    }
}

impl Vec3 {
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
    pub fn is_finite_mask (self) -> Mask3 {
        Mask3(self.0.is_finite())
    }
    
    #[inline]
    pub fn is_infinite_mask (self) -> Mask3 {
        Mask3(self.0.is_infinite())
    }
    
    #[inline]
    pub fn is_nan_mask (self) -> Mask3 {
        Mask3(self.0.is_nan())
    }

    #[inline]
    pub fn is_normal_mask (self) -> Mask3 {
        Mask3(self.0.is_normal())
    }

    #[inline]
    pub fn is_sign_positive_mask (self) -> Mask3 {
        const MASK: i32x4 = i32x4::from_array([-1, -1, -1, 0]);
        unsafe {
            Mask3(self.0.is_sign_positive() & mask32x4::from_int_unchecked(MASK))
        }
    }

    #[inline]
    pub fn is_sign_negative_mask (self) -> Mask3 {
        const MASK: i32x4 = i32x4::from_array([-1, -1, -1, 0]);
        unsafe {
            Mask3(self.0.is_sign_negative() & mask32x4::from_int_unchecked(MASK))
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Vec3 {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * f32x4::splat(rhs))
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(f32x4::splat(self) * rhs.0)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= f32x4::splat(rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / f32x4::from_array([rhs, rhs, rhs, 1.0]))
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(f32x4::from_array([self, self, self, 1.0]) / rhs.0)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= f32x4::from_array([rhs, rhs, rhs, 1.0])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mask3 {
    #[inline]
    pub fn all (self) -> bool {
        const MASK: i32x4 = i32x4::from_array([0, 0, 0, -1]);
        unsafe {
            (self.0 | mask32x4::from_int_unchecked(MASK)).all()
        }
    }

    #[inline]
    pub fn any (self) -> bool {
        self.0.any()
    }
}

impl Not for Mask3 {
    type Output = Mask3;

    #[inline]
    fn not(self) -> Self::Output {
        const MASK: i32x4 = i32x4::from_array([-1, -1, -1, 0]);
        unsafe {
            Self(self.0 ^ mask32x4::from_int_unchecked(MASK))
        }
    }
}

impl BitOr for Mask3 {
    type Output = Mask3;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for Mask3 {
    type Output = Mask3;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor for Mask3 {
    type Output = Mask3;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}