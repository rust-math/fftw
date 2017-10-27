
extern crate fftw_sys as ffi;
extern crate fftw;

use fftw::*;

fn main() {
    let n = 128;
    // Create a pair of array for out-place transform of FFTW
    let mut pair = r2hc_1d(n).to_pair().unwrap();
    // Initialize to `cos(x)` in bficient space
    pair.b.as_view_mut()[1] = 1.0;
    // execute rDCT
    pair.exec_backward();

    for val in pair.a.as_slice().iter() {
        println!("{}", val);
    }
}
