use std::{
    f64::consts::PI,
    simd::{LaneCount, Simd, SimdFloat, SimdPartialEq, SupportedLaneCount},
};

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
    let mut img = RgbImage::new(64, 64);

    for i in 0..64 {
        for j in 0..64 {
            let r = (i as f64) / 63.0;
            let b = (j as f64) / 63.0;
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

pub fn hsv_to_rgb(hsv: [f64; 3]) -> [f64; 3] {
    let h = 3.0 * hsv[0] / PI;
    let s = hsv[1];
    let v = hsv[2];

    let c = v * s;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());

    let rgb1 = match (h as usize) % 6 {
        0 => [c, x, 0.0],
        1 => [x, c, 0.0],
        2 => [0.0, c, x],
        3 => [0.0, x, c],
        4 => [x, 0.0, c],
        5 => [c, 0.0, x],
        _ => unreachable!(),
    };

    let m = v - c;

    rgb1.map(|x| x + m)
}

#[inline(always)]
fn simd_f64_mod2<const LANES: usize>(x: Simd<f64, LANES>) -> Simd<f64, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let xi: Simd<u32, LANES> = unsafe { x.to_int_unchecked() };
    let x2 = xi & Simd::splat(1);

    x2.cast() + (x - xi.cast())
}

#[inline(always)]
pub fn hsv_to_rgb_vec<const LANES: usize>(
    h: Simd<f64, LANES>,
    s: Simd<f64, LANES>,
    v: Simd<f64, LANES>,
) -> [Simd<f64, LANES>; 3]
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let h = Simd::splat(3.0 / PI) * h;

    let c = v * s;
    let x =
        c * (Simd::splat(1.0) - (simd_f64_mod2(h) - Simd::splat(1.0)).abs());

    let hi: Simd<i32, LANES> = unsafe { h.to_int_unchecked() } % Simd::splat(6);

    let hn = [0, 1, 2, 3, 4, 5].map(|x| hi.simd_eq(Simd::splat(x)).cast());

    let m = v - c;

    [((0, 5), (1, 4)), ((1, 2), (0, 3)), ((3, 4), (2, 5))].map(
        |((i, j), (k, l))| {
            (hn[i] | hn[j])
                .select(c, (hn[k] | hn[l]).select(x, Simd::splat(0.0)))
                + m
        },
    )
}

pub fn complex_to_hsv(re: f64, im: f64) {
    
}
