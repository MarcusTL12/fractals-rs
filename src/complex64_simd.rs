use std::{
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
    },
    simd::{LaneCount, Simd, StdFloat, SupportedLaneCount},
};

#[derive(Debug, Clone, Copy)]
pub struct C64Simd<const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    pub re: Simd<f64, LANES>,
    pub im: Simd<f64, LANES>,
}

impl<const LANES: usize> C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self {
            re: self.im.mul_add(-a.im, self.re.mul_add(a.re, b.re)),
            im: self.re.mul_add(a.im, self.im.mul_add(a.re, b.im)),
        }
    }

    #[inline(always)]
    pub fn splat((re, im): (f64, f64)) -> Self {
        Self {
            re: Simd::splat(re),
            im: Simd::splat(im),
        }
    }
}

impl<const LANES: usize> Add for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<const LANES: usize> AddAssign for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const LANES: usize> Neg for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<const LANES: usize> Sub for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<const LANES: usize> SubAssign for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const LANES: usize> Mul for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.im.mul_add(-rhs.im, self.re * rhs.re),
            im: self.im.mul_add(rhs.re, self.re * rhs.im),
        }
    }
}

impl<const LANES: usize> MulAssign for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const LANES: usize> Div for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.im.mul_add(rhs.im, rhs.re * rhs.re);

        Self {
            re: self.im.mul_add(rhs.im, self.re * rhs.re) / denom,
            im: self.re.mul_add(-rhs.im, self.im * rhs.re) / denom,
        }
    }
}

impl<const LANES: usize> DivAssign for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
