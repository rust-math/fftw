
extern crate fftw3_sys as ffi;
extern crate fftw3;
extern crate num_traits;

use num_traits::*;
use fftw3::*;

fn main() {
    let n = 128;
    let mut in_ = vec![c64::zero(); n];
    let mut out = vec![c64::zero(); n];
    in_[1] = c64::one();
    let plan = Plan::dft_1d(&mut in_, &mut out, SIGN::FFTW_FORWARD, FLAG::FFTW_ESTIMATE);
    plan.forward();
    println!("{:?}", plan.coef);
    plan.backward();
    println!("{:?}", plan.field);
}
