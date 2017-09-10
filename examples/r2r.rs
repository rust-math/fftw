
extern crate fftw_sys as ffi;
extern crate fftw;

use fftw::*;
use std::f64::consts::PI;

fn main() {
    let n = 128;
    let mut pair = Pair::r2r_1d(n, R2R_KIND::FFTW_R2HC, FLAG::FFTW_ESTIMATE);
    pair.coef[1] = 1.0;
    pair.backward();
    for val in pair.field.iter() {
        println!("{}", val);
    }
    let ans: Vec<f64> = (0..n)
        .map(|i| {
            let x = 2.0 * PI * i as f64 / n as f64;
            x.cos()
        })
        .collect();

    for val in ans.iter() {
        println!("{}", val);
    }
}
