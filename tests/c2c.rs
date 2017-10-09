
extern crate num_traits;
extern crate fftw;
extern crate ndarray;
#[macro_use]
extern crate ndarray_linalg;

use fftw::*;
use ndarray::*;
use ndarray_linalg::*;

/// Check successive forward and backward transform equals to the identity
fn test_identity<C: FFTWComplex>(mut pair: Pair<C, C, Ix1>, rtol: C::Real) {
    let a: Array1<C> = random(pair.size());
    println!("a = {:?}", &a);
    let b = pair.forward_array(a.view()).to_owned();
    println!("b = {:?}", &b);
    let a2 = pair.backward_array(b.view());
    println!("a2 = {:?}", &a2);
    assert_close_l2!(&a2, &a, rtol);
}

/// Check successive forward and backward transform equals to the identity
fn test_forward<C: FFTWComplex>(mut pair: Pair<C, C, Ix1>, rtol: C::Real) {
    let n = pair.size().size();
    let pi = ::std::f64::consts::PI;
    let a: Array1<C> = Array::from_iter((0..n).map(|i| {
        Scalar::from_f64((2.0 * pi * i as f64 / n as f64).cos())
    }));
    println!("a = {:?}", &a);
    let b = pair.forward_array(a.view()).to_owned();
    println!("b = {:?}", &b);
    // cos(x) = (exp(ix) + exp(-ix))/2
    let mut ans: Array1<C> = Array::zeros(b.len());
    ans[1] = Scalar::from_f64(0.5);
    ans[n - 1] = Scalar::from_f64(0.5);
    assert_close_l2!(&b, &ans, rtol);
}

mod _64 {
    use super::*;
    const N: usize = 32;
    const RTOL: f64 = 1e-7;

    #[test]
    fn identity() {
        let pair: Pair<c64, c64, Ix1> = c2c_1d(N).to_pair().unwrap();
        test_identity(pair, RTOL);
    }

    #[test]
    fn forward() {
        let pair: Pair<c64, c64, Ix1> = c2c_1d(N).to_pair().unwrap();
        test_forward(pair, RTOL);
    }
}

mod _32 {
    use super::*;
    const N: usize = 32;
    const RTOL: f32 = 1e-4;

    #[test]
    fn identity() {
        let pair: Pair<c32, c32, Ix1> = c2c_1d(N).to_pair().unwrap();
        test_identity(pair, RTOL);
    }

    #[test]
    fn forward() {
        let pair: Pair<c32, c32, Ix1> = c2c_1d(N).to_pair().unwrap();
        test_forward(pair, RTOL);
    }
}
