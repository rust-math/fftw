
extern crate fftw3;
extern crate num_traits;

use fftw3::*;
use num_traits::Zero;

#[test]
fn r2c2r() {
    let n = 128;
    let mut in_ = vec![0.0; n];
    let mut out = vec![c64::zero(); n / 2 + 1];
    let mut plan = Plan::r2c_1d(&mut in_, &mut out, FLAG::FFTW_ESTIMATE);
    for (i, val) in plan.field.iter_mut().enumerate() {
        *val = (i + 1) as f64;
    }
    plan.forward();
    plan.backward();
    for (i, val) in plan.field.iter().enumerate() {
        let ans = (i + 1) as f64;
        if (ans - *val).abs() / ans.abs() > 1e-7 {
            panic!("Not equal: ans={:?}/val={:?}", ans, val);
        }
    }
}

#[test]
fn c2r2c() {
    let n = 128;
    let mut in_ = vec![0.0; n];
    let mut out = vec![c64::zero(); n / 2 + 1];
    let mut plan = Plan::r2c_1d(&mut in_, &mut out, FLAG::FFTW_ESTIMATE);
    for (i, val) in plan.coef.iter_mut().enumerate() {
        *val = c64::new((i + 1) as f64, (i + 2) as f64);
    }
    plan.backward();
    plan.forward();
    for (i, val) in plan.coef.iter().enumerate() {
        let mut ans = c64::new((i + 1) as f64, (i + 2) as f64);
        if i == 0 || i == n / 2 {
            ans[1] = 0.0;
        }
        if (ans - *val).abs() / ans.abs() > 1e-7 {
            panic!("Not equal: i={}, ans={:?}/val={:?}", i, ans, val);
        }
    }
}
