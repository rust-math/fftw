
use ffi;
use super::enums::*;
use super::{c64, RawVec};
use super::util::FFTW_MUTEX;

pub struct Plan {
    plan: ffi::fftw_plan,
}

impl Drop for Plan {
    fn drop(&mut self) {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe {
            ffi::fftw_destroy_plan(self.plan);
        }
        drop(lock);
    }
}

impl Plan {
    pub fn r2r_1d(n: usize, in_: &mut RawVec<f64>, out: &mut RawVec<f64>, kind: R2R_KIND, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  in_.as_mut_ptr(),
                                  out.as_mut_ptr(),
                                  kind,
                                  flag as u32)
        };
        drop(lock);
        Plan { plan: plan }
    }
    pub fn r2c_1d(n: usize, in_: &mut RawVec<f64>, out: &mut RawVec<c64>, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe {
            ffi::fftw_plan_dft_r2c_1d(n as i32,
                                      in_.as_mut_ptr(),
                                      out.as_mut_ptr() as *mut ffi::fftw_complex,
                                      flag as u32)
        };
        drop(lock);
        Plan { plan: plan }
    }
    pub fn c2r_1d(n: usize, in_: &mut RawVec<c64>, out: &mut RawVec<f64>, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe {
            ffi::fftw_plan_dft_c2r_1d(n as i32,
                                      in_.as_mut_ptr() as *mut ffi::fftw_complex,
                                      out.as_mut_ptr(),
                                      flag as u32)
        };
        drop(lock);
        Plan { plan: plan }
    }
    pub fn c2c_1d(n: usize, in_: &mut RawVec<c64>, out: &mut RawVec<c64>, sign: SIGN, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe {
            ffi::fftw_plan_dft_1d(n as i32,
                                  in_.as_mut_ptr() as *mut ffi::fftw_complex,
                                  out.as_mut_ptr() as *mut ffi::fftw_complex,
                                  flag as i32,
                                  flag as u32)
        };
        drop(lock);
        Plan { plan: plan }
    }
}
