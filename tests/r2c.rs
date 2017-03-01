
extern crate fftw3;

use fftw3::*;

#[test]
fn r2c2r() {
    let n = 128;
    let mut pair = Pair::r2c_1d(n, FLAG::FFTW_ESTIMATE);
    for (i, val) in pair.field.iter_mut().enumerate() {
        *val = (i + 1) as f64;
    }
    pair.forward();
    pair.backward();
    pair.normalize_field_by(1.0 / n as f64);
    for (i, val) in pair.field.iter().enumerate() {
        let ans = (i + 1) as f64;
        if (ans - *val).abs() / ans.abs() > 1e-7 {
            panic!("Not equal: ans={:?}/val={:?}", ans, val);
        }
    }
}

#[test]
fn c2r2c() {
    let n = 128;
    let mut pair = Pair::r2c_1d(n, FLAG::FFTW_ESTIMATE);
    for (i, val) in pair.coef.iter_mut().enumerate() {
        *val = c64::new((i + 1) as f64, (i + 2) as f64);
    }
    pair.backward();
    pair.forward();
    pair.normalize_coef_by(1.0 / n as f64);
    for (i, val) in pair.coef.iter().enumerate() {
        let mut ans = c64::new((i + 1) as f64, (i + 2) as f64);
        if i == 0 || i == n / 2 {
            ans[1] = 0.0;
        }
        if (ans - *val).abs() / ans.abs() > 1e-7 {
            panic!("Not equal: i={}, ans={:?}/val={:?}", i, ans, val);
        }
    }
}
