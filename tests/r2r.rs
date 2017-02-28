
extern crate fftw3;
extern crate float_cmp;

use fftw3::*;
use float_cmp::ApproxEqRatio;

#[test]
fn r2hc2r() {
    let n = 128;
    let mut in_ = vec![0.0; n];
    let mut out = vec![0.0; n];
    let mut plan = Plan::r2r_1d(&mut in_, &mut out, R2R_KIND::FFTW_R2HC, FLAG::FFTW_ESTIMATE);
    for (i, val) in plan.field.iter_mut().enumerate() {
        *val = (i + 1) as f64;
    }
    plan.forward();
    plan.backward();
    for (i, val) in plan.field.iter().enumerate() {
        let ans = (i + 1) as f64;
        if !val.approx_eq_ratio(&ans, 1e-7) {
            panic!("Not equal: ans={}/val={}", ans, val);
        }
    }
}

#[test]
fn hc2r2hc() {
    let n = 128;
    let mut in_ = vec![0.0; n];
    let mut out = vec![0.0; n];
    let mut plan = Plan::r2r_1d(&mut in_, &mut out, R2R_KIND::FFTW_R2HC, FLAG::FFTW_ESTIMATE);
    for (i, val) in plan.coef.iter_mut().enumerate() {
        *val = (i + 1) as f64;
    }
    plan.backward();
    plan.forward();
    for (i, val) in plan.coef.iter().enumerate() {
        let ans = (i + 1) as f64;
        if !val.approx_eq_ratio(&ans, 1e-7) {
            panic!("Not equal: ans={}/val={}", ans, val);
        }
    }
}
