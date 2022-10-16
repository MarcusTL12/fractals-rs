use std::simd::{LaneCount, SupportedLaneCount};

use crate::complex64_simd::C64Simd;

#[inline(always)]
pub fn polyval<const LANES: usize, const N: usize>(
    x: C64Simd<LANES>,
    p: &[(f64, f64); N],
) -> C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut xn = C64Simd::splat((1.0, 0.0));
    let mut acc = C64Simd::splat((0.0, 0.0));

    for &c in p {
        acc = xn.mul_add(C64Simd::splat(c), acc);
        xn *= x;
    }

    acc
}

#[inline(always)]
pub fn polyder<const N: usize>(p: &[(f64, f64); N + 1]) -> [(f64, f64); N] {
    let mut dp = [(0.0, 0.0); N];

    for ((i, (cr, ci)), (dcr, dci)) in p.iter().enumerate().skip(1).zip(&mut dp)
    {
        *dcr = cr * i as f64;
        *dci = ci * i as f64;
    }

    dp
}

pub fn newton<const LANES: usize, const D: usize, const ITERS: usize>(
    mut x: C64Simd<LANES>,
    p: &[(f64, f64); D + 1],
) -> C64Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let dp: [_; D] = polyder(p);

    for _ in 0..ITERS {
        x -= polyval(x, p) / polyval(x, &dp);
    }

    x
}
