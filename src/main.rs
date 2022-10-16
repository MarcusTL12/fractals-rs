#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::{simd::Simd, time::Instant};

use complex64_simd::C64Simd;
use polynomials::newton;

pub mod complex64_simd;
pub mod polynomials;

fn main() {
    let p = [(0.0, 0.0), (-3.0, 0.0), (0.0, 0.0), (1.0, 0.0)];

    let x = C64Simd {
        re: Simd::from([1.5, 2.5, 2.0, 2.0, 1.5, 1.5, 2.5, 2.5]),
        im: Simd::from([0.0, 0.0, 0.5, -0.5, 0.5, -0.5, 0.5, -0.5]),
    };

    let t = Instant::now();
    let x = newton::<8, 3, 1000000000>(x, &p);
    let t = t.elapsed();

    println!("{x:?}\ntook {t:?}");
}
