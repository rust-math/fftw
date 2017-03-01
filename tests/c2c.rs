
extern crate fftw;

use fftw::*;

#[test]
fn c2c2c() {
    let n = 128;
    let mut pair = Pair::c2c_1d(n, SIGN::FFTW_FORWARD, FLAG::FFTW_ESTIMATE);
    for (i, val) in pair.field.iter_mut().enumerate() {
        *val = c64::new((i + 1) as f64, (i + 2) as f64);
    }
    pair.forward();
    pair.backward();
    pair.normalize_field_by(1.0 / n as f64);
    for (i, val) in pair.field.iter().enumerate() {
        let ans = c64::new((i + 1) as f64, (i + 2) as f64);
        if (ans - *val).abs() / ans.abs() > 1e-7 {
            panic!("Not equal: ans={:?}/val={:?}", ans, val);
        }
    }
}
