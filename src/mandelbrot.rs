use std::simd::{LaneCount, Simd, SimdPartialOrd, SupportedLaneCount};

use crate::complex64_simd::{C64Simd, C64SimdSelect};

pub fn mandelbrot<const LANES: usize, const ITERS: usize>(
    mut z: C64Simd<LANES>,
    c: C64Simd<LANES>,
) -> (C64Simd<LANES>, Simd<i64, LANES>)
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut counter = Simd::splat(0);

    for _ in 0..ITERS {
        let m = z.abssqr().simd_ge(Simd::splat(4.0));
        counter += m.select(Simd::splat(0), Simd::splat(1));
        z = m.cselect(z, z.mul_add(z, c))
    }

    (z, counter)
}
