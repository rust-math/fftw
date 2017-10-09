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
    pub a: AlignedVec<A>,
    pub b: AlignedVec<B>,
    pub(crate) size: D,
    pub(crate) forward: RawPlan,
    pub(crate) backward: RawPlan,
    // normaliztion factors
    // `None` means no normaliztion
    pub(crate) factor_f: Option<A::Real>,
    pub(crate) factor_b: Option<B::Real>,
}

impl<A, B> Pair<A, B, Ix1>
where
    A: Scalar,
    B: Scalar<Real = A::Real>,
{
    /// Execute `Pair::forward` with `ndarray::ArrayView`
    pub fn forward_array<'a, 'b>(&'a mut self, input: ArrayView1<'b, A>) -> ArrayViewMut1<'a, B> {
        let sl = self.forward(input.as_slice().unwrap());
        ArrayViewMut::from(sl)
    }

    /// Execute `Pair::backward` with `ndarray::ArrayView`
    pub fn backward_array<'a, 'b>(&'a mut self, input: ArrayView1<'b, B>) -> ArrayViewMut1<'a, A> {
        let sl = self.backward(input.as_slice().unwrap());
        ArrayViewMut::from(sl)
    }
}

impl<A, B, D: Dimension> Pair<A, B, D>
where
    A: Scalar,
    B: Scalar<Real = A::Real>,
{
    pub fn size(&self) -> D {
        self.size.clone()
    }

    /// Executes copy the input to `a`, forward transform,
    /// and returns the result `b` as a reference
    pub fn forward(&mut self, input: &[A]) -> &mut [B] {
        self.a.copy_from_slice(input);
        self.exec_forward();
        if let Some(n) = self.factor_f.as_ref() {
            for val in self.b.iter_mut() {
                *val = val.mul_real(*n);
            }
        }
        &mut self.b
    }

    /// Execute copy to pair, forward transform,
    /// and returns a reference of the result.
    pub fn backward(&mut self, input: &[B]) -> &mut [A]
    where
        A: Scalar,
        B: Scalar,
    {
        self.b.copy_from_slice(input);
        self.exec_backward();
        if let Some(n) = self.factor_b.as_ref() {
            for val in self.a.iter_mut() {
                *val = val.mul_real(*n);
            }
        }
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
pub trait ToPair<A, B>
where
    A: Scalar,
    B: Scalar<Real = A::Real>,
{
    type Dim: Dimension;
    /// Generate `Pair` from a setting struct
    fn to_pair(&self) -> Result<Pair<A, B, Self::Dim>>;
}
