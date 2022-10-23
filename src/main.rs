#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use fractal_image::test_grad;

pub mod mandelbrot;
pub mod polynomials;
pub mod fractal_image;

fn main() {
    let x: f64 = 3.14;

    let y = x.exp();

    println!("{y}");
}
