use std::simd::{LaneCount, Simd, SimdFloat, SupportedLaneCount};

use image::{Rgb, RgbImage};

pub fn float_to_byte_vec<const LANES: usize>(
    x: Simd<f64, LANES>,
) -> Simd<u8, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    (Simd::splat(255.0) * x.simd_clamp(Simd::splat(0.0), Simd::splat(1.0)))
        .cast()
}

pub fn make_grad() {
    let mut img = RgbImage::new(10, 10);

    // img.put_pixel(5, 5, Rgb([1.0, 0.0, 1.0]));

    img.save("test.png").unwrap();
}
