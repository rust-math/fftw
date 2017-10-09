
extern crate num_traits;
extern crate fftw;
extern crate ndarray;
#[macro_use]
extern crate ndarray_linalg;

use fftw::*;
use ndarray::*;
use ndarray_linalg::*;

/// Check successive forward and backward transformation conserves.
fn test_identity<R, C>(mut pair: Pair<R, C, Ix1>, rtol: R::Real)
where
    R: FFTWReal,
    C: FFTWComplex<Real = R::Real>,
{
    let a: Array1<R> = random(pair.size());
    println!("a = {:?}", &a);
    let b = {
        let b = pair.forward(a.as_slice().unwrap());
        Array::from_vec(b.to_vec())
    };
    println!("b = {:?}", &b);
    let a2 = pair.backward(b.as_slice().unwrap());
    let a2: Array1<R> = Array::from_vec(a2.to_vec());
    println!("a2 = {:?}", &a2);
    assert_close_l2!(&a2, &a, rtol);
}

/// Check `cos(k_0 x)` is transformed `b[1] = 1.0 + 0.0i`
fn test_forward<R, C>(mut pair: Pair<R, C, Ix1>, rtol: C::Real)
where
    R: FFTWReal,
    C: FFTWComplex<Real = R::Real>,
{
    let n = pair.size().size();
    let pi = ::std::f64::consts::PI;
    let a: Array1<R> = Array::from_iter((0..n).map(|i| {
        Scalar::from_f64((2.0 * pi * i as f64 / n as f64).cos())
    }));
    println!("a = {:?}", &a);
    let b = {
        let b = pair.forward(a.as_slice().unwrap());
        Array::from_vec(b.to_vec())
    };
    println!("b = {:?}", &b);
    let mut ans: Array1<C> = Array::zeros(b.len());
    ans[1] = Scalar::from_f64(0.5); // cos(x) = 0.5*exp(ix) + c.c.
    assert_close_l2!(&b, &ans, rtol);
}

mod _64 {
    use super::*;
    const N: usize = 32;
    const RTOL: f64 = 1e-4;

    #[test]
    fn identity() {
        let pair: Pair<f64, c64, Ix1> = r2c_1d(N).to_pair().unwrap();
        test_identity(pair, RTOL);
    }

    #[test]
    fn forward() {
        let pair: Pair<f64, c64, Ix1> = r2c_1d(N).to_pair().unwrap();
        test_forward(pair, RTOL);
    }
}

mod _32 {
    use super::*;
    const N: usize = 32;
    const RTOL: f32 = 1e-4;

    #[test]
    fn identity() {
        let pair: Pair<f32, c32, Ix1> = r2c_1d(N).to_pair().unwrap();
        test_identity(pair, RTOL);
    }

    #[test]
    fn forward() {
        let pair: Pair<f32, c32, Ix1> = r2c_1d(N).to_pair().unwrap();
        test_forward(pair, RTOL);
    }
}
