//! Safe-interface corresponding to out-place transform

use super::raw_vec::*;
use super::plan::*;
use super::enums::*;
use super::r2r::*;

use num_traits::Zero;

/// Safe-interface corresponding to out-place transform
///
/// FFTW interface modifies an array in `fftw_execute` function
/// which does not takes the array as its arguments.
/// It is not compatible to the programing model of safe Rust.
/// `Pair` interface composes the array and plan to manage
/// mutability in the safe Rust way.
pub struct Pair<A, B> {
    pub field: RawVec<A>,
    pub coef: RawVec<B>,
    logical_size: usize,
    forward: Plan<A, B>,
    backward: Plan<B, A>,
}

impl<A, B> Pair<A, B> {
    pub fn logical_size(&self) -> usize { self.logical_size }

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

impl<R> Pair<R, R>
    where R: R2RPlanCreate + AlignedAllocable + Zero
{
    /// Create one-dimensional Real-to-Real transformation pair
    pub fn r2r_1d(n: usize, kind: R2R_KIND, flag: FLAG) -> Self {
        let mut field = RawVec::new(n);
        let mut coef = RawVec::new(n);
        let forward = Plan::r2r_1d(n, &mut field, &mut coef, forward(kind), flag);
        let backward = Plan::r2r_1d(n, &mut coef, &mut field, backward(kind), flag);
        Pair {
            field: field,
            coef: coef,
            logical_size: logical_size(n, kind),
            forward: forward,
            backward: backward,
        }
    }
}

impl<C> Pair<C, C>
    where C: C2CPlanCreate + AlignedAllocable + Zero
{
    /// Create one-dimensional Complex-to-Complex transformation pair
    pub fn c2c_1d(n: usize, sign: SIGN, flag: FLAG) -> Self {
        let mut field = RawVec::new(n);
        let mut coef = RawVec::new(n);
        let forward = Plan::c2c_1d(n, &mut field, &mut coef, sign, flag);
        let backward = Plan::c2c_1d(n, &mut coef, &mut field, -sign, flag);
        Pair {
            field: field,
            coef: coef,
            logical_size: n,
            forward: forward,
            backward: backward,
        }
    }
}

impl<R, C> Pair<R, C>
    where (C, R): C2RPlanCreate<Real = R, Complex = C>,
          R: AlignedAllocable + Zero,
          C: AlignedAllocable + Zero
{
    /// Create one-dimensional Real-to-Complex transformation pair
    pub fn r2c_1d(n: usize, flag: FLAG) -> Self {
        let mut field = RawVec::<R>::new(n);
        let mut coef = RawVec::<C>::new(n / 2 + 1);
        let forward = Plan::r2c_1d(n, &mut field, &mut coef, flag);
        let backward = Plan::c2r_1d(n, &mut coef, &mut field, flag);
        Pair {
            field: field,
            coef: coef,
            logical_size: n,
            forward: forward,
            backward: backward,
        }
    }
}
