//! Safe-interface corresponding to out-place transform

use super::aligned_vec::*;
use super::error::*;
use super::plan::*;

use ndarray::*;
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

    pub fn forward_transform(&mut self, input: &[A]) -> &[B]
    where
        A: Copy,
    {
        self.field.copy_from_slice(input);
        self.forward();
        &self.coef
    }

    pub fn backward_transform(&mut self, input: &[B]) -> &[A]
    where
        B: Copy,
    {
        self.coef.copy_from_slice(input);
        self.backward();
        &self.field
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
