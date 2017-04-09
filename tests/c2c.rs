
extern crate fftw;
extern crate num_complex;

macro_rules! impl_test{
    ($modname:ident, $float:ident, $complex:ident, $th:expr) => {

mod $modname {

use fftw::*;

#[test]
fn c2c2c() {
    let n = 128;
    let mut pair = Pair::c2c_1d(n, SIGN::FFTW_FORWARD, FLAG::FFTW_ESTIMATE);
    for (i, val) in pair.field.iter_mut().enumerate() {
        *val = $complex::new((i + 1) as $float, (i + 2) as $float);
    }
    pair.forward();
    pair.backward();
    for x in pair.field.iter_mut() {
        *x = *x / n as $float;
    }
    for (i, val) in pair.field.iter().enumerate() {
        let ans = $complex::new((i + 1) as $float, (i + 2) as $float);
        if (ans - *val).norm() / ans.norm() > $th {
            panic!("Not equal: ans={:?}/val={:?}", ans, val);
        }
    }
}

} // mod
}} // impl_test

impl_test!(_32, f32, c32, 1e-4);
impl_test!(_64, f64, c64, 1e-7);
