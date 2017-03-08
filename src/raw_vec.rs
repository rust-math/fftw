
use ffi;
use super::c64;
use super::util::FFTW_MUTEX;

use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::os::raw::c_void;

pub struct RawVec<T> {
    n: usize,
    data: *mut T,
}

impl<T> RawVec<T> {
    pub fn as_slice(&self) -> &[T] { unsafe { from_raw_parts(self.data, self.n) } }
    pub fn as_mut_slice(&mut self) -> &mut [T] { unsafe { from_raw_parts_mut(self.data, self.n) } }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe { ffi::fftw_free(self.data as *mut c_void) };
        drop(lock);
    }
}

impl RawVec<f64> {
    pub fn new(n: usize) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let ptr = unsafe { ffi::fftw_alloc_real(n) };
        drop(lock);
        RawVec { n: n, data: ptr }
    }
}

impl RawVec<c64> {
    pub fn new(n: usize) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let ptr = unsafe { ffi::fftw_alloc_complex(n) } as *mut c64;
        drop(lock);
        RawVec { n: n, data: ptr }
    }
}
