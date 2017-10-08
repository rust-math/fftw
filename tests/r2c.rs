
extern crate fftw;
extern crate num_complex;

macro_rules! impl_test{
    ($modname:ident, $float:ident, $complex:ident, $th:expr) => {

mod $modname {

use fftw::*;

#[test]
fn r2c2r() {
    let n = 128;
    let mut pair = r2c_1d(n).to_pair().unwrap();
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
        if (ans - *val).abs() / ans.abs() > $th {
            panic!("Not equal: ans={:?}/val={:?}", ans, val);
        }
    }
}

#[test]
fn c2r2c() {
    let n = 128;
    let mut pair = r2c_1d(n).to_pair().unwrap();
    for (i, val) in pair.b.iter_mut().enumerate() {
        *val = $complex::new((i + 1) as $float, (i + 2) as $float);
    }
    pair.exec_backward();
    pair.exec_forward();
    for x in pair.b.iter_mut() {
        *x = *x / n as $float;
    }
    for (i, val) in pair.b.iter().enumerate() {
        let mut ans = $complex::new((i + 1) as $float, (i + 2) as $float);
        if i == 0 || i == n / 2 {
            ans.im = 0.0;
        }
        if (ans - *val).norm() / ans.norm() > $th {
            panic!("Not equal: i={}, ans={:?}/val={:?}", i, ans, val);
        }
    }
}

} // mod
}} // impl_test

impl_test!(_32, f32, c32, 1e-4);
impl_test!(_64, f64, c64, 1e-7);
