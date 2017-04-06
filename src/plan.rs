
use ffi;
use super::enums::*;
use super::raw_vec::RawVec;
use super::util::FFTW_MUTEX;

use num_complex::Complex64 as c64;
use std::marker::PhantomData;

pub struct Plan<A, B> {
    plan: ffi::fftw_plan,
    phantom: PhantomData<(A, B)>,
}

impl<A, B> Drop for Plan<A, B> {
    fn drop(&mut self) {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe {
            ffi::fftw_destroy_plan(self.plan);
        }
        drop(lock);
    }
}

impl<A, B> Plan<A, B> {
    /// this function modifys the array referred in plan creation
    pub unsafe fn execute(&self) { ffi::fftw_execute(self.plan); }

    pub fn r2c_1d(n: usize, in_: &mut RawVec<f64>, out: &mut RawVec<c64>, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe { ffi::fftw_plan_dft_r2c_1d(n as i32, in_.as_mut_ptr(), out.as_mut_ptr(), flag as u32) };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }

    pub fn c2r_1d(n: usize, in_: &mut RawVec<c64>, out: &mut RawVec<f64>, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe { ffi::fftw_plan_dft_c2r_1d(n as i32, in_.as_mut_ptr(), out.as_mut_ptr(), flag as u32) };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }

    pub fn c2c_1d(n: usize, in_: &mut RawVec<c64>, out: &mut RawVec<c64>, sign: SIGN, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe {
            ffi::fftw_plan_dft_1d(n as i32,
                                  in_.as_mut_ptr(),
                                  out.as_mut_ptr(),
                                  sign as i32,
                                  flag as u32)
        };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }
}

impl Plan<f64, f64> {
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
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }
}

impl Plan<f32, f32> {
    pub fn r2r_1d(n: usize, in_: &mut RawVec<f32>, out: &mut RawVec<f32>, kind: R2R_KIND, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe {
            ffi::fftwf_plan_r2r_1d(n as i32,
                                   in_.as_mut_ptr(),
                                   out.as_mut_ptr(),
                                   kind,
                                   flag as u32)
        };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }
}
