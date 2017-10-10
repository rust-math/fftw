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
    pub(crate) a: (AlignedVec<A>, Shape<D>),
    pub(crate) b: (AlignedVec<B>, Shape<D>),
    pub(crate) forward: RawPlan,
    pub(crate) backward: RawPlan,
    // normaliztion factors
    // `None` means no normaliztion
    pub(crate) factor_f: Option<A::Real>,
    pub(crate) factor_b: Option<B::Real>,
}

impl<A, B, D: Dimension> Pair<A, B, D>
where
    A: Scalar,
    B: Scalar<Real = A::Real>,
{
    pub fn a_shape(&self) -> Shape<D> {
        self.a.1.clone()
    }

    pub fn b_shape(&self) -> Shape<D> {
        self.b.1.clone()
    }

    pub fn get_a<'a>(&'a self) -> Result<ArrayView<'a, A, D>> {
        Ok(ArrayView::from_shape(self.a_shape(), &self.a.0)?)
    }

    pub fn get_b<'a>(&'a self) -> Result<ArrayView<'a, B, D>> {
        Ok(ArrayView::from_shape(self.b_shape(), &self.b.0)?)
    }

    pub fn get_a_mut<'a>(&'a mut self) -> Result<ArrayViewMut<'a, A, D>> {
        Ok(ArrayViewMut::from_shape(self.a_shape(), &mut self.a.0)?)
    }

    pub fn get_b_mut<'a>(&'a mut self) -> Result<ArrayViewMut<'a, B, D>> {
        Ok(ArrayViewMut::from_shape(self.b_shape(), &mut self.b.0)?)
    }

    // /// Execute `Pair::forward` with `ndarray::ArrayView`
    // pub fn forward_array<'a, 'b>(&'a mut self, input: ArrayView<'b, A, D>) -> ArrayViewMut<'a, B, D> {
    //     let sl = self.forward(input.as_slice().unwrap());
    //     ArrayViewMut::from(sl).into_shape(dim).unwrap()
    // }
    //
    // /// Execute `Pair::backward` with `ndarray::ArrayView`
    // pub fn backward_array<'a, 'b>(&'a mut self, input: ArrayView<'b, B, D>) -> ArrayViewMut<'a, A, D> {
    //     let dim = self.a_dim();
    //     let sl = self.backward(input.as_slice().unwrap());
    //     ArrayViewMut::from(sl).into_shape(dim).unwrap()
    // }

    /// Executes copy the input to `a`, forward transform,
    /// and returns the result `b` as a reference
    pub fn forward(&mut self, input: &[A]) -> &mut [B] {
        self.a.0.copy_from_slice(input);
        self.exec_forward();
        if let Some(n) = self.factor_f.as_ref() {
            for val in self.b.0.iter_mut() {
                *val = val.mul_real(*n);
            }
        }
        &mut self.b.0
    }

    /// Execute copy to pair, forward transform,
    /// and returns a reference of the result.
    pub fn backward(&mut self, input: &[B]) -> &mut [A]
    where
        A: Scalar,
        B: Scalar,
    {
        self.b.0.copy_from_slice(input);
        self.exec_backward();
        if let Some(n) = self.factor_b.as_ref() {
            for val in self.a.0.iter_mut() {
                *val = val.mul_real(*n);
            }
        }
        &mut self.a.0
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
