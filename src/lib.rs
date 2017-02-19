
extern crate fftw3_sys as ffi;

use ffi::fftw_r2r_kind;

fn create_plan() {
    let n = 128;
    let mut in_ = vec![0.0; n];
    let mut out = vec![0.0; n];
    unsafe {
        ffi::fftw_plan_r2r_1d(n as i32,
                              in_.as_mut_ptr(),
                              out.as_mut_ptr(),
                              fftw_r2r_kind::FFTW_R2HC,
                              0);
    }
}
