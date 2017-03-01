
use ffi;
use super::plan::Plan;
use super::{R2R_KIND, FLAG, SIGN, c64};
use super::util::FFTW_MUTEX;

use num_traits::Zero;
use std::slice::from_raw_parts_mut;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;

/// Field and Coefficient pair
///
/// - This struct is a wrapper of `Plan`
/// - This struct uses fftw_malloc to enable SIMD optimization in FFTW.
pub struct Pair<'a, 'b, A, B>
    where A: 'a,
          B: 'b
{
    plan: Plan<'a, 'b, A, B>,
}

impl<'a, 'b, A, B> Drop for Pair<'a, 'b, A, B> {
    fn drop(&mut self) {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe {
            ffi::fftw_free(self.plan.field.as_mut_ptr() as *mut c_void);
            ffi::fftw_free(self.plan.coef.as_mut_ptr() as *mut c_void);
        }
        drop(lock);
    }
}

impl<'a, 'b, A, B> Deref for Pair<'a, 'b, A, B> {
    type Target = Plan<'a, 'b, A, B>;
    fn deref(&self) -> &Self::Target {
        &self.plan
    }
}

impl<'a, 'b, A, B> DerefMut for Pair<'a, 'b, A, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.plan
    }
}

fn alloc_real<'a>(n: usize) -> &'a mut [f64] {
    let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
    let slice = unsafe {
        let ptr = ffi::fftw_alloc_real(n);
        from_raw_parts_mut(ptr, n)
    };
    drop(lock);
    for val in slice.iter_mut() {
        *val = 0.0;
    }
    slice
}

fn alloc_complex<'a>(n: usize) -> &'a mut [c64] {
    let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
    let slice = unsafe {
        let ptr = ffi::fftw_alloc_complex(n) as *mut c64;
        from_raw_parts_mut(ptr, n)
    };
    drop(lock);
    for val in slice.iter_mut() {
        *val = c64::zero();
    }
    slice
}

impl<'a, 'b> Pair<'a, 'b, f64, f64> {
    pub fn r2r_1d(n: usize, kind: R2R_KIND, flag: FLAG) -> Self {
        let field = alloc_real(n);
        let coef = alloc_real(n);
        let plan = Plan::r2r_1d(field, coef, kind, flag);
        Pair { plan: plan }
    }
}

impl<'a, 'b> Pair<'a, 'b, f64, c64> {
    pub fn r2c_1d(n: usize, flag: FLAG) -> Self {
        let field = alloc_real(n);
        let coef = alloc_complex(n / 2 + 1);
        let plan = Plan::r2c_1d(field, coef, flag);
        Pair { plan: plan }
    }
}

impl<'a, 'b> Pair<'a, 'b, c64, c64> {
    pub fn c2c_1d(n: usize, sign: SIGN, flag: FLAG) -> Self {
        let field = alloc_complex(n);
        let coef = alloc_complex(n);
        let plan = Plan::c2c_1d(field, coef, sign, flag);
        Pair { plan: plan }
    }
}
