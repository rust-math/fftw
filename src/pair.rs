//! Safe-interface corresponding to out-place transform

use super::aligned_vec::*;
use super::error::*;
use super::plan::*;

use ndarray::*;
use ndarray_linalg::Scalar;

/// Safe-interface corresponding to out-place transform
///
/// FFTW interface modifies an array in `fftw_execute` function
/// which does not takes the array as its arguments.
/// It is not compatible to the programing model of safe Rust.
/// `Pair` interface composes the array and plan to manage
/// mutability in the safe Rust way.
pub struct Pair<A, B, D: Dimension>
where
    A: Scalar,
    B: Scalar<Real = A::Real>,
{
    pub a: AlignedArray<A, D>,
    pub b: AlignedArray<B, D>,
    pub(crate) forward: Plan<B>,
    pub(crate) backward: Plan<A>,
}

impl<A, B, D: Dimension> Pair<A, B, D>
where
    A: Scalar,
    B: Scalar<Real = A::Real>,
{
    /// Execute `Pair::forward` with `ndarray::ArrayView`
    pub fn forward_array<'a, 'b>(&'a mut self, input: ArrayView<'b, A, D>) -> ArrayViewMut<'a, B, D> {
        self.a.as_view_mut().assign(&input);
        self.exec_forward();
        self.b.as_view_mut()
    }

    /// Execute `Pair::backward` with `ndarray::ArrayView`
    pub fn backward_array<'a, 'b>(&'a mut self, input: ArrayView<'b, B, D>) -> ArrayViewMut<'a, A, D> {
        self.b.as_view_mut().assign(&input);
        self.exec_backward();
        self.a.as_view_mut()
    }

    /// Executes copy the input to `a`, forward transform,
    /// and returns the result `b` as a reference
    pub fn forward(&mut self, input: &[A]) -> &mut [B] {
        self.a.copy_from_slice(input);
        self.exec_forward();
        self.b.as_slice_mut()
    }

    /// Execute copy to pair, forward transform,
    /// and returns a reference of the result.
    pub fn backward(&mut self, input: &[B]) -> &mut [A] {
        self.b.copy_from_slice(input);
        self.exec_backward();
        self.a.as_slice_mut()
    }

    /// Execute a forward transform (`a` to `b`)
    pub fn exec_forward(&mut self) {
        unsafe { self.forward.execute() }
        self.forward.normalize(self.b.as_slice_mut());
    }

    /// Execute a backward transform (`b` to `a`)
    pub fn exec_backward(&mut self) {
        unsafe { self.backward.execute() }
        self.backward.normalize(self.a.as_slice_mut());
    }

    pub(crate) fn null_checked(self) -> Result<Self> {
        self.forward.check_null()?;
        self.backward.check_null()?;
        Ok(self)
    }
}

/// Create a `Pair` from a setting struct e.g. `R2C1D`.
pub trait ToPair<A, B>
where
    A: Scalar,
    B: Scalar<Real = A::Real>,
{
    type Dim: Dimension;
    /// Generate `Pair` from a setting struct
    fn to_pair(&self) -> Result<Pair<A, B, Self::Dim>>;
}
