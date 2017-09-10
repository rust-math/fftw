use super::{c32, c64};
use super::util::FFTW_MUTEX;
use ffi;

use num_traits::Zero;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::os::raw::c_void;
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub struct AlignedVec<T> {
    n: usize,
    data: *mut T,
}

pub trait AlignedAllocable {
    unsafe fn alloc(n: usize) -> *mut Self;
}

impl AlignedAllocable for f64 {
    unsafe fn alloc(n: usize) -> *mut Self {
        ffi::fftw_alloc_real(n)
    }
}

impl AlignedAllocable for f32 {
    unsafe fn alloc(n: usize) -> *mut Self {
        ffi::fftwf_alloc_real(n)
    }
}

impl AlignedAllocable for c64 {
    unsafe fn alloc(n: usize) -> *mut Self {
        ffi::fftw_alloc_complex(n)
    }
}

impl AlignedAllocable for c32 {
    unsafe fn alloc(n: usize) -> *mut Self {
        ffi::fftwf_alloc_complex(n)
    }
}

impl<T> AlignedVec<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { from_raw_parts(self.data, self.n) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { from_raw_parts_mut(self.data, self.n) }
    }
}

impl<T> Deref for AlignedVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for AlignedVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> Drop for AlignedVec<T> {
    fn drop(&mut self) {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe { ffi::fftw_free(self.data as *mut c_void) };
        drop(lock);
    }
}

impl<T> AlignedVec<T>
where
    T: Zero + AlignedAllocable,
{
    pub fn new(n: usize) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let ptr = unsafe { T::alloc(n) };
        drop(lock);
        let mut vec = AlignedVec { n: n, data: ptr };
        for v in vec.iter_mut() {
            *v = T::zero();
        }
        vec
    }
}

impl<T> Index<usize> for AlignedVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.data.offset(index as isize) }
    }
}

impl<T> IndexMut<usize> for AlignedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.data.offset(index as isize) }
    }
}
