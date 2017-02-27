
use ffi;
use super::r2r::*;
use super::{c64, FLAG, SIGN};
use std::ops::MulAssign;

#[derive(Debug)]
pub struct Plan<'a, 'b, A, B>
    where A: 'a,
          B: 'b
{
    pub field: &'a mut [A],
    pub coef: &'b mut [B],
    logical_size: usize,
    forward: ffi::fftw_plan,
    backward: ffi::fftw_plan,
}

impl<'a, 'b, A, B> Plan<'a, 'b, A, B> {
    /// [field] -> [coef]
    pub fn forward(&mut self) {
        unsafe {
            ffi::fftw_execute(self.forward);
        }
    }
    /// [field] <- [coef]
    pub fn backward(&mut self) {
        unsafe {
            ffi::fftw_execute(self.backward);
        }
    }
    pub fn normalize(&mut self)
        where A: MulAssign<f64>
    {
        let n = 1.0 / self.logical_size as f64;
        for val in self.field.iter_mut() {
            *val *= n;
        }
    }
}

impl<'a, 'b, A, B> Drop for Plan<'a, 'b, A, B> {
    fn drop(&mut self) {
        unsafe {
            ffi::fftw_destroy_plan(self.forward);
            ffi::fftw_destroy_plan(self.backward);
        }
    }
}

impl<'a, 'b> Plan<'a, 'b, f64, f64> {
    pub fn r2r_1d(field: &'a mut [f64], coef: &'b mut [f64], kind: R2R_KIND, flag: FLAG) -> Self {
        let n = field.len();
        let forward = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  field.as_mut_ptr(),
                                  coef.as_mut_ptr(),
                                  forward(kind),
                                  flag as u32)
        };
        let backward = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  coef.as_mut_ptr(),
                                  field.as_mut_ptr(),
                                  backward(kind),
                                  flag as u32)
        };
        Plan {
            field: field,
            coef: coef,
            logical_size: logical_size(n, kind),
            forward: forward,
            backward: backward,
        }
    }
}

impl<'a, 'b> Plan<'a, 'b, c64, c64> {
    pub fn dft_1d(field: &'a mut [c64], coef: &'b mut [c64], sign: SIGN, flag: FLAG) -> Self {
        let n = field.len();
        let forward = unsafe {
            ffi::fftw_plan_dft_1d(n as i32,
                                  field.as_mut_ptr() as *mut ffi::fftw_complex,
                                  coef.as_mut_ptr() as *mut ffi::fftw_complex,
                                  sign as i32,
                                  flag as u32)
        };
        let backward = unsafe {
            ffi::fftw_plan_dft_1d(n as i32,
                                  coef.as_mut_ptr() as *mut ffi::fftw_complex,
                                  field.as_mut_ptr() as *mut ffi::fftw_complex,
                                  -(sign as i32),
                                  flag as u32)
        };
        Plan {
            field: field,
            coef: coef,
            logical_size: n,
            forward: forward,
            backward: backward,
        }

    }
}

impl<'a, 'b> Plan<'a, 'b, f64, c64> {
    pub fn r2c_1d(field: &'a mut [f64], coef: &'b mut [c64], flag: FLAG) -> Self {
        let n = field.len();
        let forward = unsafe {
            ffi::fftw_plan_dft_r2c_1d(n as i32,
                                      field.as_mut_ptr(),
                                      coef.as_mut_ptr() as *mut ffi::fftw_complex,
                                      flag as u32)
        };
        let backward = unsafe {
            ffi::fftw_plan_dft_c2r_1d(n as i32,
                                      coef.as_mut_ptr() as *mut ffi::fftw_complex,
                                      field.as_mut_ptr(),
                                      flag as u32)
        };
        Plan {
            field: field,
            coef: coef,
            logical_size: n,
            forward: forward,
            backward: backward,
        }

    }
}
