
extern crate fftw;
extern crate float_cmp;

use fftw::*;
use float_cmp::ApproxEqRatio;
use std::f64::consts::PI;

#[test]
fn r2hc2r() {
    let n = 128;
    let mut pair = Pair::r2r_1d(n, R2R_KIND::FFTW_R2HC, FLAG::FFTW_ESTIMATE);
    for (i, val) in pair.field.iter_mut().enumerate() {
        *val = (i + 1) as f64;
    }
    pair.forward();
    pair.backward();
    pair.normalize_field_by(1.0 / n as f64);
    for (i, val) in pair.field.iter().enumerate() {
        let ans = (i + 1) as f64;
        if !val.approx_eq_ratio(&ans, 1e-7) {
            panic!("Not equal: ans={}/val={}", ans, val);
        }
    }
}

#[test]
fn hc2r2hc() {
    let n = 128;
    let mut pair = Pair::r2r_1d(n, R2R_KIND::FFTW_R2HC, FLAG::FFTW_ESTIMATE);
    for (i, val) in pair.coef.iter_mut().enumerate() {
        *val = (i + 1) as f64;
    }
    pair.backward();
    pair.forward();
    pair.normalize_coef_by(1.0 / n as f64);
    for (i, val) in pair.coef.iter().enumerate() {
        let ans = (i + 1) as f64;
        if !val.approx_eq_ratio(&ans, 1e-7) {
            panic!("Not equal: ans={}/val={}", ans, val);
        }
    }
}

#[test]
fn hc2r() {
    let n = 128;
    let mut pair = Pair::r2r_1d(n, R2R_KIND::FFTW_R2HC, FLAG::FFTW_ESTIMATE);
    pair.coef[0] = 2.0;
    pair.coef[1] = 1.0;
    pair.backward();
    let ans: Vec<f64> = (0..n)
        .map(|i| {
            let x = 2.0 * PI * i as f64 / n as f64;
            2.0 + 2.0 * x.cos()
        })
        .collect();
    for (v, a) in pair.field.iter().zip(ans.iter()) {
        if (v - a).abs() > 1e-7 {
            panic!("Not equal: ans={}/val={}", a, v);
        }
    }
}
