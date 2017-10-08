
extern crate fftw;
extern crate ndarray;
extern crate ndarray_linalg;

use fftw::*;
use ndarray::*;
use ndarray_linalg::*;

fn test_r2c2r<R, C>(mut pair: Pair<R, C, Ix1>, rtol: R::Real)
where
    R: FFTWReal + Scalar,
    C: FFTWComplex + Scalar,
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

#[test]
fn r2c2r() {
    let n = 32;
    let pair: Pair<f64, c64, Ix1> = r2c_1d(n).to_pair().unwrap();
    test_r2c2r(pair, 1e-7);
}
