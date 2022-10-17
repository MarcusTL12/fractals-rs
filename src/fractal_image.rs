use std::simd::{LaneCount, Simd, SimdFloat, SupportedLaneCount};

use image::{Rgb, RgbImage};

pub fn float_to_byte(x: f64) -> u8 {
    (255.0 * x.clamp(0.0, 1.0)) as u8
}

#[inline(always)]
pub fn float_to_byte_vec<const LANES: usize>(
    x: Simd<f64, LANES>,
) -> Simd<u8, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    (Simd::splat(255.0) * x.simd_clamp(Simd::splat(0.0), Simd::splat(1.0)))
        .cast()
}

pub fn test_grad() {
    let mut img = RgbImage::new(8, 8);

    for i in 0..8 {
        for j in 0..8 {
            let r = (i as f64) / 7.0;
            let b = (j as f64) / 7.0;
            let g = 1.0 - (r + b) / 2.0;

            img.put_pixel(
                i,
                j,
                Rgb([float_to_byte(r), float_to_byte(g), float_to_byte(b)]),
            );
        }
    }

    img.save("test.png").unwrap();
}
