use std::simd::{LaneCount, Simd, SimdPartialOrd, SupportedLaneCount};

use complex_simd::{SimdComplex, SimdComplexSelect};

pub fn mandelbrot<const LANES: usize, const ITERS: usize>(
    mut z: SimdComplex<Simd<f64, LANES>>,
    c: SimdComplex<Simd<f64, LANES>>,
) -> (SimdComplex<Simd<f64, LANES>>, Simd<i64, LANES>)
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
