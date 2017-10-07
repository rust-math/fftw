//! Safe-interface corresponding to out-place transform

use super::aligned_vec::*;
use super::error::*;
use super::plan::*;

use ndarray::*;

/// Safe-interface corresponding to out-place transform
///
/// FFTW interface modifies an array in `fftw_execute` function
/// which does not takes the array as its arguments.
/// It is not compatible to the programing model of safe Rust.
/// `Pair` interface composes the array and plan to manage
/// mutability in the safe Rust way.
pub struct Pair<A, B, D: Dimension> {
    pub a: AlignedVec<A>,
    pub b: AlignedVec<B>,
    pub size: D::Pattern,
    pub(crate) forward: RawPlan,
    pub(crate) backward: RawPlan,
    // normaliztion factors
    // `None` means no normaliztion
    pub(crate) factor_f: Option<B>,
    pub(crate) factor_b: Option<A>,
}

impl<A, B, D: Dimension> Pair<A, B, D> {
    /// Executes copy the input to `a`, forward transform,
    /// and returns the result `b` as a reference
    pub fn forward(&mut self, input: &[A]) -> &mut [B]
    where
        A: Copy,
    {
        self.a.copy_from_slice(input);
        &mut self.b
    }

    /// Execute copy to pair, forward transform,
    /// and returns a reference of the result.
    pub fn backward(&mut self, input: &[B]) -> &mut [A]
    where
        B: Copy,
    {
        self.b.copy_from_slice(input);
        self.exec_backward();
        &mut self.a
    }

    /// Execute a forward transform (`a` to `b`)
    pub fn exec_forward(&mut self) {
        unsafe { self.forward.execute() }
    }

    /// Execute a backward transform (`b` to `a`)
    pub fn exec_backward(&mut self) {
        unsafe { self.backward.execute() }
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
