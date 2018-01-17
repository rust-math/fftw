extern crate fftw;
extern crate ndarray;
#[macro_use]
extern crate ndarray_linalg;
extern crate num_traits;

use fftw::*;
use ndarray::*;
use ndarray_linalg::*;

/// Check successive forward and backward transformation conserves.
fn test_identity<R>(mut pair: Pair<R, R, Ix1>, rtol: R::Real)
where
    R: FFTWReal,
{
    let a: Array1<R> = random(pair.a.dim());
    println!("a = {:?}", &a);
    let b = pair.forward_array(a.view()).to_owned();
    println!("b = {:?}", &b);
    let a2 = pair.backward_array(b.view());
    println!("a2 = {:?}", &a2);
    assert_close_l2!(&a2, &a, rtol);
}

/// Check `cos(k_0 x)` is transformed `b[1] = 1.0 + 0.0i`
fn test_forward<R>(mut pair: Pair<R, R, Ix1>, rtol: R::Real)
where
    R: FFTWReal,
{
    let n = pair.a.dim();
    let pi = ::std::f64::consts::PI;
    let a: Array1<R> =
        Array::from_iter((0..n).map(|i| Scalar::from_f64((2.0 * pi * i as f64 / n as f64).cos())));
    println!("a = {:?}", &a);
    let b = pair.forward_array(a.view()).to_owned();
    println!("b = {:?}", &b);
    let mut ans: Array1<R> = Array::zeros(b.len());
    ans[1] = Scalar::from_f64(0.5); // cos(x) = 0.5*exp(ix) + c.c.
    assert_close_l2!(&b, &ans, rtol);
}

mod r2r_64 {
    use super::*;
    const N: usize = 32;
    const RTOL: f64 = 1e-7;

    #[cfg_attr(feature = "intel-mkl", should_panic)]
    #[test]
    fn r2hc_identity() {
        let pair: Pair<f64, f64, Ix1> = r2hc_1d(N).to_pair().unwrap();
        test_identity(pair, RTOL);
    }

    #[cfg_attr(feature = "intel-mkl", should_panic)]
    #[test]
    fn r2hc_forward() {
        let pair: Pair<f64, f64, Ix1> = r2hc_1d(N).to_pair().unwrap();
        test_forward(pair, RTOL);
    }
}

mod r2r_32 {
    use super::*;
    const N: usize = 32;
    const RTOL: f32 = 1e-4;

    #[cfg_attr(feature = "intel-mkl", should_panic)]
    #[test]
    fn r2hc_identity() {
        let pair: Pair<f32, f32, Ix1> = r2hc_1d(N).to_pair().unwrap();
        test_identity(pair, RTOL);
    }

    #[cfg_attr(feature = "intel-mkl", should_panic)]
    #[test]
    fn r2hc_forward() {
        let pair: Pair<f32, f32, Ix1> = r2hc_1d(N).to_pair().unwrap();
        test_forward(pair, RTOL);
    }
}
