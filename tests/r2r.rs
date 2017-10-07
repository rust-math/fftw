
extern crate fftw;
extern crate float_cmp;

macro_rules! impl_test{
    ($modname:ident, $float:ident, $th:expr) => {

mod $modname {

use fftw::*;
use float_cmp::ApproxEqRatio;
use std::$float::consts::PI;

#[cfg_attr(feature = "intel-mkl", should_panic)]
#[test]
fn r2hc2r() {
    let n = 128;
    let mut pair = r2hc_1d(n).to_pair();
    for (i, val) in pair.field.iter_mut().enumerate() {
        *val = (i + 1) as $float;
    }
    pair.forward();
    pair.backward();
    for x in pair.field.iter_mut() {
        *x /= n as $float;
    }
    for (i, val) in pair.field.iter().enumerate() {
        let ans = (i + 1) as $float;
        if !val.approx_eq_ratio(&ans, $th) {
            panic!("Not equal: ans={}/val={}", ans, val);
        }
    }
}

#[cfg_attr(feature = "intel-mkl", should_panic)]
#[test]
fn hc2r2hc() {
    let n = 128;
    let mut pair = r2hc_1d(n).to_pair();
    for (i, val) in pair.coef.iter_mut().enumerate() {
        *val = (i + 1) as $float;
    }
    pair.backward();
    pair.forward();
    for x in pair.coef.iter_mut() {
        *x /= n as $float;
    }
    for (i, val) in pair.coef.iter().enumerate() {
        let ans = (i + 1) as $float;
        if !val.approx_eq_ratio(&ans, $th) {
            panic!("Not equal: ans={}/val={}", ans, val);
        }
    }
}

#[cfg_attr(feature = "intel-mkl", should_panic)]
#[test]
fn hc2r() {
    let n = 128;
    let mut pair = r2hc_1d(n).to_pair();
    pair.coef[0] = 2.0;
    pair.coef[1] = 1.0;
    pair.backward();
    let ans: Vec<$float> = (0..n)
        .map(|i| {
            let x = 2.0 * PI * i as $float / n as $float;
            2.0 + 2.0 * x.cos()
        })
        .collect();
    for (v, a) in pair.field.iter().zip(ans.iter()) {
        if (v - a).abs() > $th {
            panic!("Not equal: ans={}/val={}", a, v);
        }
    }
}

} // mod
}} // impl_test

impl_test!(_32, f32, 1e-4);
impl_test!(_64, f64, 1e-7);
