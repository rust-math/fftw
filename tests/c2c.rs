
extern crate fftw3;
extern crate num_traits;

use fftw3::*;
use num_traits::Zero;

#[test]
fn c2c2c() {
    let n = 128;
    let mut in_ = vec![c64::zero(); n];
    let mut out = vec![c64::zero(); n];
    let mut plan = Plan::dft_1d(&mut in_, &mut out, SIGN::FFTW_FORWARD, FLAG::FFTW_ESTIMATE);
    for (i, val) in plan.field.iter_mut().enumerate() {
        *val = c64::new((i + 1) as f64, (i + 2) as f64);
    }
    plan.forward();
    plan.backward();
    for (i, val) in plan.field.iter().enumerate() {
        let ans = c64::new((i + 1) as f64, (i + 2) as f64);
        if (ans - *val).abs() / ans.abs() > 1e-7 {
            panic!("Not equal: ans={:?}/val={:?}", ans, val);
        }
    }
}
