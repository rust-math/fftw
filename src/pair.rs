//! Safe-interface corresponding to out-place transform

use super::aligned_vec::*;
use super::enums::*;
use super::plan::*;
use super::r2r::*;

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
    logical_size: usize,
    forward: RawPlan,
    backward: RawPlan,
    phantom: PhantomData<D>,
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
}

/// Create a `Pair` from a setting struct e.g. `R2C1D`.
pub trait ToPair<A, B> {
    type Dim: Dimension;
    /// Generate `Pair` from a setting struct
    fn to_pair(&self) -> Pair<A, B, Self::Dim>;
}

#[derive(Debug, Clone, Copy, new)]
pub struct R2R1D {
    n: usize,
    kind: R2R_KIND,
    flag: FLAG,
}

pub fn r2hc_1d(n: usize) -> R2R1D {
    R2R1D {
        n: n,
        kind: R2R_KIND::FFTW_R2HC,
        flag: FLAG::FFTW_MEASURE,
    }
}

impl<T: R2R + AlignedAllocable + Zero> ToPair<T, T> for R2R1D {
    type Dim = Ix1;
    fn to_pair(&self) -> Pair<T, T, Ix1> {
        let mut field = AlignedVec::new(self.n);
        let mut coef = AlignedVec::new(self.n);
        let forward = unsafe { T::r2r_1d(self.n, &mut field, &mut coef, forward(self.kind), self.flag) };
        let backward = unsafe {
            T::r2r_1d(
                self.n,
                &mut coef,
                &mut field,
                backward(self.kind),
                self.flag,
            )
        };
        Pair {
            field: field,
            coef: coef,
            logical_size: logical_size(self.n, self.kind),
            forward: forward,
            backward: backward,
            phantom: PhantomData,
        }
    }
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
        flag: FLAG::FFTW_MEASURE,
    }
}

impl<T: C2C + AlignedAllocable + Zero> ToPair<T, T> for C2C1D {
    type Dim = Ix1;
    fn to_pair(&self) -> Pair<T, T, Ix1> {
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
        }
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
        flag: FLAG::FFTW_MEASURE,
    }
}

impl<R, C> ToPair<R, C> for R2C1D
where
    (C, R): C2R<Real = R, Complex = C>,
    R: AlignedAllocable + Zero,
    C: AlignedAllocable + Zero,
{
    type Dim = Ix1;
    fn to_pair(&self) -> Pair<R, C, Ix1> {
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
        }
    }
}
