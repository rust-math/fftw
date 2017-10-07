//! Safe-interface corresponding to out-place transform

use super::{FLAG, SIGN};
use super::aligned_vec::*;
use super::error::*;
use super::plan::*;

use ffi;

use ndarray::*;
use num_traits::Zero;
use std::marker::PhantomData;

/// Safe-interface corresponding to out-place transform
///
/// FFTW interface modifies an array in `fftw_execute` function
/// which does not takes the array as its arguments.
/// It is not compatible to the programing model of safe Rust.
/// `Pair` interface composes the array and plan to manage
/// mutability in the safe Rust way.
pub struct Pair<A, B, D> {
    pub field: AlignedVec<A>,
    pub coef: AlignedVec<B>,
    pub(crate) logical_size: usize,
    pub(crate) forward: RawPlan,
    pub(crate) backward: RawPlan,
    pub(crate) phantom: PhantomData<D>,
}

impl<A, B, D: Dimension> Pair<A, B, D> {
    pub fn logical_size(&self) -> usize {
        self.logical_size
    }

    /// Execute forward transformation
    pub fn forward(&mut self) {
        unsafe {
            self.forward.execute();
        }
    }

    /// Execute backward transformation
    pub fn backward(&mut self) {
        unsafe {
            self.backward.execute();
        }
    }

    pub(crate) fn null_checked(self) -> Result<Self> {
        if self.forward.is_null() || self.backward.is_null() {
            Err(InvalidPlanError {}.into())
        } else {
            Ok(self)
        }
    }
}

/// Create a `Pair` from a setting struct e.g. `R2C1D`.
pub trait ToPair<A, B> {
    type Dim: Dimension;
    /// Generate `Pair` from a setting struct
    fn to_pair(&self) -> Result<Pair<A, B, Self::Dim>>;
}

/// Setting for 1-dimensional C2C transform
#[derive(Debug, Clone, Copy, new)]
pub struct C2C1D {
    n: usize,
    sign: SIGN,
    flag: FLAG,
}

/// Utility function to generage 1-dimensional C2C setting
pub fn c2c_1d(n: usize) -> C2C1D {
    C2C1D {
        n,
        sign: SIGN::FFTW_FORWARD,
        flag: ffi::FFTW_MEASURE,
    }
}

impl<T: C2C + AlignedAllocable + Zero> ToPair<T, T> for C2C1D {
    type Dim = Ix1;
    fn to_pair(&self) -> Result<Pair<T, T, Ix1>> {
        let mut field = AlignedVec::new(self.n);
        let mut coef = AlignedVec::new(self.n);
        let forward = unsafe { T::c2c_1d(self.n, &mut field, &mut coef, self.sign, self.flag) };
        let backward = unsafe { T::c2c_1d(self.n, &mut coef, &mut field, -self.sign, self.flag) };
        Pair {
            field: field,
            coef: coef,
            logical_size: self.n,
            forward: forward,
            backward: backward,
            phantom: PhantomData,
        }.null_checked()
    }
}

/// Setting for 1-dimensional R2C transform
#[derive(Debug, Clone, Copy, new)]
pub struct R2C1D {
    n: usize,
    flag: FLAG,
}

/// Utility function to generage 1-dimensional R2C setting
pub fn r2c_1d(n: usize) -> R2C1D {
    R2C1D {
        n,
        flag: ffi::FFTW_MEASURE,
    }
}

impl<R, C> ToPair<R, C> for R2C1D
where
    (C, R): C2R<Real = R, Complex = C>,
    R: AlignedAllocable + Zero,
    C: AlignedAllocable + Zero,
{
    type Dim = Ix1;
    fn to_pair(&self) -> Result<Pair<R, C, Ix1>> {
        let mut field = AlignedVec::<R>::new(self.n);
        let mut coef = AlignedVec::<C>::new(self.n / 2 + 1);
        let forward = unsafe { <(C, R) as C2R>::r2c_1d(self.n, &mut field, &mut coef, self.flag) };
        let backward = unsafe { <(C, R) as C2R>::c2r_1d(self.n, &mut coef, &mut field, self.flag) };
        Pair {
            field: field,
            coef: coef,
            logical_size: self.n,
            forward: forward,
            backward: backward,
            phantom: PhantomData,
        }.null_checked()
    }
}
