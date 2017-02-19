
extern crate fftw3_sys as ffi;
extern crate fftw3;

use fftw3::*;

fn main() {
    let n = 128;
    let mut in_ = vec![0.0; n];
    let mut out = vec![0.0; n];
    let plan = Plan::r2r_1d(&mut in_, &mut out, R2R_KIND::FFTW_HC2R, FLAG::FFTW_ESTIMATE);
    plan.coef[0] = 1.0;
    plan.coef[1] = 1.0;
    plan.coef[n - 1] = 1.0;
    plan.backward();
    for (i, val) in plan.field.iter().enumerate() {
        println!("field[{}] = {}", i, val);
    }
    plan.forward();
    for (i, val) in plan.coef.iter().enumerate() {
        println!("coef[{}] = {}", i, val);
    }
}
