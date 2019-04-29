extern crate fftw;
extern crate num_traits;

use fftw::plan::*;
use fftw::types::*;
use num_traits::Zero;

/// Check successive forward and backward transform equals to the identity
#[test]
fn r2r2r_identity() {
    let n = 32;
    let mut a = vec![0.0f64; n];
    let mut b = vec![0.0f64; n];

    // http://www.fftw.org/fftw3_doc/Real_002dto_002dReal-Transform-Kinds.html,
    // contains the normalization rules for each R2R transform. Given
    // normalization `N`, we can test `inverse_dct(dct(x / N)) == x`.
    // If you want to apply the normalization symmetrically in the transform
    // and inverse, then simply divide all elements of the output (or input)
    // vector by `sqrt(N)`, both on the forward and inverse passes. We
    // demonstrate this here.

    // Here we use a type-2 DCT, whose inverse is a type-3 DCT, and whose
    // normalization is `N=2*n`.
    let mut fwd: R2RPlan64 =
        R2RPlan::new(&[n], &mut a, &mut b, R2RKind::FFTW_REDFT10, Flag::Measure).unwrap();
    let mut bwd: R2RPlan64 =
        R2RPlan::new(&[n], &mut b, &mut a, R2RKind::FFTW_REDFT01, Flag::Measure).unwrap();
    let N = 2. * n as f64;

    // Vector of ones.
    a = vec![1.0f64; n];

    // Forward pass.
    fwd.r2r(&mut a, &mut b).unwrap();
    // Renormalize
    for mut i in &mut b {
        *i /= N.sqrt();
    }

    // Inverse.
    bwd.r2r(&mut b, &mut a).unwrap();
    // Renormalize.
    for mut i in &mut a {
        *i /= N.sqrt();
    }

    // Ensure we have the original vector of ones.
    for v in a.iter() {
        let dif = (v - 1.).abs();
        if dif > 1e-7 {
            panic!("Large difference: v={}, dif={}", v, dif);
        }
    }
}
