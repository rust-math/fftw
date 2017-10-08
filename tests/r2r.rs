
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
    let mut pair = r2hc_1d(n).to_pair().unwrap();
    for (i, val) in pair.a.iter_mut().enumerate() {
        *val = (i + 1) as $float;
    }
    pair.exec_forward();
    pair.exec_backward();
    for x in pair.a.iter_mut() {
        *x /= n as $float;
    }
    for (i, val) in pair.a.iter().enumerate() {
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
    let mut pair = r2hc_1d(n).to_pair().unwrap();
    for (i, val) in pair.b.iter_mut().enumerate() {
        *val = (i + 1) as $float;
    }
    pair.exec_backward();
    pair.exec_forward();
    for x in pair.b.iter_mut() {
        *x /= n as $float;
    }
    for (i, val) in pair.b.iter().enumerate() {
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
    let mut pair = r2hc_1d(n).to_pair().unwrap();
    pair.b[0] = 2.0;
    pair.b[1] = 1.0;
    pair.exec_backward();
    let ans: Vec<$float> = (0..n)
        .map(|i| {
            let x = 2.0 * PI * i as $float / n as $float;
            2.0 + 2.0 * x.cos()
        })
        .collect();
    for (v, a) in pair.a.iter().zip(ans.iter()) {
        if (v - a).abs() > $th {
            panic!("Not equal: ans={}/val={}", a, v);
        }
    }
}

} // mod
}} // impl_test

impl_test!(_32, f32, 1e-4);
impl_test!(_64, f64, 1e-7);
