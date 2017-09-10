
extern crate fftw_sys as ffi;
extern crate fftw;

use fftw::*;

fn main() {
    let n = 128;
    // Create a pair of array for out-place transform of FFTW
    let mut pair = Pair::r2r_1d(n, R2R_KIND::FFTW_R2HC, FLAG::FFTW_ESTIMATE);
    // Initialize to `cos(x)` in coefficient space
    pair.coef[1] = 1.0;
    // execute rDCT
    pair.backward();

    for val in pair.field.iter() {
        println!("{}", val);
    }
}
