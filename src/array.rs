use types::*;
use error::*;
use ffi;

use ndarray::*;
use num_traits::Zero;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::os::raw::c_void;
use std::slice::{from_raw_parts, from_raw_parts_mut};

#[derive(Debug, Clone)]
pub struct AlignedArray<A, D>
where
    A: AlignedAllocable,
    D: Dimension,
{
    data: AlignedVec<A>,
    shape: Shape<D>,
}

impl<A: AlignedAllocable> AlignedArray<A, Ix1> {
    pub fn from_vec(data: AlignedVec<A>) -> Self {
        let shape = data.len().into_shape();
        Self { data, shape }
    }
}

impl<A: AlignedAllocable, D: Dimension> AlignedArray<A, D> {
    pub fn new<Sh: ShapeBuilder<Dim = D>>(shape: Sh) -> Self
    where
        A: Zero + AlignedAllocable,
    {
        let shape = shape.into_shape();
        let data = AlignedVec::new(shape.size());
        Self { data, shape }
    }

    pub fn dim(&self) -> D::Pattern {
        self.as_view().dim()
    }

    pub fn shape(&self) -> &Shape<D> {
        &self.shape
    }

    pub fn copy_from_slice(&mut self, input: &[A])
    where
        A: Copy,
    {
        self.data.copy_from_slice(input)
    }

    pub fn as_slice(&self) -> &[A] {
        self.data.as_slice()
    }

    pub fn as_slice_mut(&mut self) -> &mut [A] {
        self.data.as_slice_mut()
    }

    pub fn as_view<'a>(&'a self) -> ArrayView<'a, A, D> {
        self.data.as_view(self.shape.clone()).unwrap()
    }

    pub fn as_view_mut<'a>(&'a mut self) -> ArrayViewMut<'a, A, D> {
        self.data.as_view_mut(self.shape.clone()).unwrap()
    }
}

/// Array with SIMD alignment
///
/// This wraps `fftw_alloc` and `fftw_free`for SIMD feature
/// http://www.fftw.org/fftw3_doc/SIMD-alignment-and-fftw_005fmalloc.html
#[derive(Debug)]
pub struct AlignedVec<T> {
    n: usize,
    data: *mut T,
}

pub trait AlignedAllocable: Zero {
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
    /// Recast to Rust's immutable slice
    pub fn as_slice(&self) -> &[T] {
        unsafe { from_raw_parts(self.data, self.n) }
    }
    /// Recast to Rust's mutable slice
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { from_raw_parts_mut(self.data, self.n) }
    }

    pub fn as_view<'a, Sh, D>(&'a self, shape: Sh) -> Result<ArrayView<'a, T, D>>
    where
        D: Dimension,
        Sh: ShapeBuilder<Dim = D>,
    {
        Ok(ArrayView::from_shape(shape, self)?)
    }

    pub fn as_view_mut<'a, Sh, D>(&'a mut self, shape: Sh) -> Result<ArrayViewMut<'a, T, D>>
    where
        D: Dimension,
        Sh: ShapeBuilder<Dim = D>,
    {
        Ok(ArrayViewMut::from_shape(shape, self)?)
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
        self.as_slice_mut()
    }
}

impl<T> Drop for AlignedVec<T> {
    fn drop(&mut self) {
        excall! { ffi::fftw_free(self.data as *mut c_void) };
    }
}

impl<T> AlignedVec<T>
where
    T: AlignedAllocable,
{
    /// Create array with `fftw_malloc` (`fftw_free` is automatically called when the arrya is `Drop`-ed)
    pub fn new(n: usize) -> Self {
        let ptr = excall! { T::alloc(n) };
        let mut vec = AlignedVec { n: n, data: ptr };
        for v in vec.iter_mut() {
            *v = T::zero();
        }
        vec
    }
}

impl<T> Clone for AlignedVec<T>
where
    T: AlignedAllocable,
{
    fn clone(&self) -> Self {
        Self::new(self.n)
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
