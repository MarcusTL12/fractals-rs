#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use fractal_image::test_grad;

pub mod complex64_simd;
pub mod mandelbrot;
pub mod polynomials;
pub mod fractal_image;

fn main() {
    // const D: usize = 5;

    // let p = [
    //     (-1.0, 0.0),
    //     (0.0, 0.0),
    //     (0.0, 0.0),
    //     (0.0, 0.0),
    //     (0.0, 0.0),
    //     (1.0, 0.0),
    // ];
    // let dp = polyder(&p);

    // let x = C64Simd {
    //     re: Simd::from([-0.5, 0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.5])
    //         * Simd::splat(5.0),
    //     im: Simd::from([0.0, 0.0, 0.5, -0.5, 0.5, -0.5, 0.5, -0.5])
    //         * Simd::splat(5.0),
    // };

    // let t = Instant::now();
    // let x = newton_checked::<8, D, 10>(x, &p, &dp, 1000, 0.0);
    // let t = t.elapsed();

    // println!("{:8.5?}\n{:8.5?}\ntook {t:?}", x.re, x.im);

    test_grad();
}
