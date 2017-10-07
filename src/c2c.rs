use super::FLAG;
use super::aligned_vec::*;
use super::error::*;
use super::pair::{Pair, ToPair};
use super::plan::C2C;

use ffi;
pub use ffi::SIGN;

use ndarray::*;
use num_traits::Zero;
use std::marker::PhantomData;

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
        let mut a = AlignedVec::new(self.n);
        let mut b = AlignedVec::new(self.n);
        let forward = unsafe { T::c2c_1d(self.n, &mut a, &mut b, self.sign, self.flag) };
        let backward = unsafe { T::c2c_1d(self.n, &mut b, &mut a, -self.sign, self.flag) };
        Pair {
            a,
            b,
            forward,
            backward,
            phantom: PhantomData,
        }.null_checked()
    }
}
