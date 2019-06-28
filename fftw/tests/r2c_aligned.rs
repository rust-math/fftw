use fftw::array::*;
use fftw::plan::*;
use fftw::types::*;

/// Check successive forward and backward transform equals to the identity
#[test]
fn c2r2c_identity() {
    let n = 32;
    let mut c2r: C2RPlan64 = C2RPlan::aligned(&[n], Flag::Measure).unwrap();
    let mut r2c: R2CPlan64 = R2CPlan::aligned(&[n], Flag::Measure).unwrap();
    let mut a = AlignedVec::new(n / 2 + 1);
    let mut b = AlignedVec::new(n);
    for i in 0..(n / 2 + 1) {
        a[i] = c64::new(1.0, 0.0);
    }
    c2r.c2r(&mut a, &mut b).unwrap();
    r2c.r2c(&mut b, &mut a).unwrap();
    for v in a.iter() {
        let ans = c64::new(n as f64, 0.0);
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!("Large difference: v={}, dif={}", v, dif);
        }
    }
}
