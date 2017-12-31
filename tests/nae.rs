
extern crate fftw;
extern crate num_traits;

use fftw::*;
use num_traits::Zero;

/// Check successive forward and backward transform equals to the identity
#[test]
fn nae_c2c2c_identity() {
    let n = 32;
    let mut a = vec![c64::zero(); n];
    let mut b = vec![c64::zero(); n];
    let mut plan = nae::C2CPlan::new(&[n], &mut a, &mut b, Sign::FFTW_FORWARD, FFTW_MEASURE).unwrap();
    for i in 0..n {
        a[i] = c64::new(1.0, 0.0);
    }
    plan.c2c(&mut a, &mut b).unwrap();
    plan.c2c(&mut b, &mut a).unwrap();
    for v in a.iter() {
        let ans = c64::new(n as f64, 0.0);
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!("Large difference: v={}, dif={}", v, dif);
        }
    }
}

/// Check cos transform
#[test]
fn nae_c2c_cos() {
    let n = 32;
    let mut a = vec![c64::zero(); n];
    let mut b = vec![c64::zero(); n];
    let mut plan = nae::C2CPlan::new(&[n], &mut a, &mut b, Sign::FFTW_FORWARD, FFTW_MEASURE).unwrap();
    let pi = ::std::f64::consts::PI;
    for i in 0..n {
        a[i] = c64::new((2.0 * pi * i as f64 / n as f64).cos(), 0.0);
    }
    plan.c2c(&mut a, &mut b).unwrap();
    for (i, v) in b.iter().enumerate() {
        let ans = if i == 1 || i == n - 1 {
            0.5 * n as f64
        } else {
            0.0
        };
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!(
                "Large difference: v={}, ans={}, dif={}, i={}",
                v,
                ans,
                dif,
                i
            );
        }
    }
}

/// Check successive forward and backward transform equals to the identity
#[test]
fn nae_c2r2c_identity() {
    let n = 32;
    let mut a = vec![c64::zero(); n / 2 + 1];
    let mut b = vec![0.0; n];
    let mut c2r = nae::C2RPlan::new(&[n], &mut a, &mut b, FFTW_MEASURE).unwrap();
    let mut r2c = nae::R2CPlan::new(&[n], &mut b, &mut a, FFTW_MEASURE).unwrap();
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
