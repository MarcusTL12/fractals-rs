use std::{
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
    },
    simd::{LaneCount, Mask, Simd, StdFloat, SupportedLaneCount},
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

    #[inline(always)]
    pub fn abssqr(self) -> Simd<f64, LANES> {
        self.im.mul_add(self.im, self.re * self.re)
    }

    #[inline(always)]
    pub fn abs(self) -> Simd<f64, LANES> {
        self.abssqr().sqrt()
    }

    #[inline(always)]
    pub fn select(self, other: Self, m: Mask<i64, LANES>) -> Self {
        Self {
            re: m.select(self.re, other.re),
            im: m.select(self.im, other.im),
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

impl<const LANES: usize> Add<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Simd<f64, LANES>) -> Self::Output {
        Self {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

impl<const LANES: usize> Add<C64Simd<LANES>> for Simd<f64, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = C64Simd<LANES>;

    #[inline(always)]
    fn add(self, rhs: C64Simd<LANES>) -> Self::Output {
        rhs + self
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

impl<const LANES: usize> AddAssign<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Simd<f64, LANES>) {
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

impl<const LANES: usize> Sub<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Simd<f64, LANES>) -> Self::Output {
        Self {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

impl<const LANES: usize> Sub<C64Simd<LANES>> for Simd<f64, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = C64Simd<LANES>;

    #[inline(always)]
    fn sub(self, rhs: C64Simd<LANES>) -> Self::Output {
        C64Simd {
            re: self - rhs.re,
            im: -rhs.im,
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

impl<const LANES: usize> SubAssign<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Simd<f64, LANES>) {
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

impl<const LANES: usize> Mul<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Simd<f64, LANES>) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl<const LANES: usize> Mul<C64Simd<LANES>> for Simd<f64, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = C64Simd<LANES>;

    #[inline(always)]
    fn mul(self, rhs: C64Simd<LANES>) -> Self::Output {
        rhs * self
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

impl<const LANES: usize> MulAssign<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Simd<f64, LANES>) {
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
        let denom = rhs.abssqr();

        Self {
            re: self.im.mul_add(rhs.im, self.re * rhs.re) / denom,
            im: self.re.mul_add(-rhs.im, self.im * rhs.re) / denom,
        }
    }
}

impl<const LANES: usize> Div<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Simd<f64, LANES>) -> Self::Output {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl<const LANES: usize> Div<C64Simd<LANES>> for Simd<f64, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = C64Simd<LANES>;

    #[inline(always)]
    fn div(self, rhs: C64Simd<LANES>) -> Self::Output {
        let denom = rhs.abssqr();

        C64Simd {
            re: self * rhs.re / denom,
            im: -self * rhs.im / denom,
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

impl<const LANES: usize> DivAssign<Simd<f64, LANES>> for C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Simd<f64, LANES>) {
        *self = *self / rhs;
    }
}
