
extern crate fftw3_sys as ffi;
extern crate fftw3;

use fftw3::*;

fn main() {
    let n = 128;
    let mut in_ = vec![0.0; n];
    in_[0] = 1.0;
    let mut out = vec![0.0; n];
    let plan = Plan::r2r_1d(&mut in_, &mut out, R2R_KIND::FFTW_HC2R, FLAG::FFTW_ESTIMATE);
    plan.execute();
    plan.input[n - 1] = 2.0;
    plan.execute();
    for val in plan.output.iter() {
        println!("{:?}", val);
    }
}
