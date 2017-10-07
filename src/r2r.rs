use super::FLAG;
use super::aligned_vec::*;
use super::error::*;
use super::pair::{Pair, ToPair};
use super::plan::R2R;

use ffi;
pub use ffi::fftw_r2r_kind as R2R_KIND;

use ndarray::*;
use num_traits::Zero;

fn forward(kind: R2R_KIND) -> R2R_KIND {
    match kind {
        R2R_KIND::FFTW_R2HC => R2R_KIND::FFTW_R2HC,
        R2R_KIND::FFTW_HC2R => R2R_KIND::FFTW_R2HC,
        R2R_KIND::FFTW_DHT => R2R_KIND::FFTW_DHT,
        R2R_KIND::FFTW_REDFT00 => R2R_KIND::FFTW_REDFT00,
        R2R_KIND::FFTW_REDFT01 => R2R_KIND::FFTW_REDFT10,
        R2R_KIND::FFTW_REDFT10 => R2R_KIND::FFTW_REDFT10,
        R2R_KIND::FFTW_REDFT11 => R2R_KIND::FFTW_REDFT11,
        R2R_KIND::FFTW_RODFT00 => R2R_KIND::FFTW_RODFT00,
        R2R_KIND::FFTW_RODFT01 => R2R_KIND::FFTW_RODFT10,
        R2R_KIND::FFTW_RODFT10 => R2R_KIND::FFTW_RODFT10,
        R2R_KIND::FFTW_RODFT11 => R2R_KIND::FFTW_RODFT11,
    }
}

fn backward(kind: R2R_KIND) -> R2R_KIND {
    match kind {
        R2R_KIND::FFTW_R2HC => R2R_KIND::FFTW_HC2R,
        R2R_KIND::FFTW_HC2R => R2R_KIND::FFTW_HC2R,
        R2R_KIND::FFTW_DHT => R2R_KIND::FFTW_DHT,
        R2R_KIND::FFTW_REDFT00 => R2R_KIND::FFTW_REDFT00,
        R2R_KIND::FFTW_REDFT01 => R2R_KIND::FFTW_REDFT01,
        R2R_KIND::FFTW_REDFT10 => R2R_KIND::FFTW_REDFT01,
        R2R_KIND::FFTW_REDFT11 => R2R_KIND::FFTW_REDFT11,
        R2R_KIND::FFTW_RODFT00 => R2R_KIND::FFTW_RODFT00,
        R2R_KIND::FFTW_RODFT01 => R2R_KIND::FFTW_RODFT01,
        R2R_KIND::FFTW_RODFT10 => R2R_KIND::FFTW_RODFT01,
        R2R_KIND::FFTW_RODFT11 => R2R_KIND::FFTW_RODFT11,
    }
}

/// see http://www.fftw.org/fftw3_doc/Real_002dto_002dReal-Transform-Kinds.html
fn logical_size(n: usize, kind: R2R_KIND) -> usize {
    match kind {
        R2R_KIND::FFTW_R2HC => n,
        R2R_KIND::FFTW_HC2R => n,
        R2R_KIND::FFTW_DHT => n,
        R2R_KIND::FFTW_REDFT00 => 2 * (n - 1),
        R2R_KIND::FFTW_REDFT01 => 2 * n,
        R2R_KIND::FFTW_REDFT10 => 2 * n,
        R2R_KIND::FFTW_REDFT11 => 2 * n,
        R2R_KIND::FFTW_RODFT00 => 2 * (n + 1),
        R2R_KIND::FFTW_RODFT01 => 2 * n,
        R2R_KIND::FFTW_RODFT10 => 2 * n,
        R2R_KIND::FFTW_RODFT11 => 2 * n,
    }
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
        flag: ffi::FFTW_MEASURE,
    }
}

impl<T: R2R + AlignedAllocable + Zero> ToPair<T, T> for R2R1D {
    type Dim = Ix1;
    fn to_pair(&self) -> Result<Pair<T, T, Ix1>> {
        let mut a = AlignedVec::new(self.n);
        let mut b = AlignedVec::new(self.n);
        let forward = unsafe { T::r2r_1d(self.n, &mut a, &mut b, forward(self.kind), self.flag) };
        let backward = unsafe { T::r2r_1d(self.n, &mut b, &mut a, backward(self.kind), self.flag) };
        Pair {
            a,
            b,
            size: self.n,
            forward,
            backward,
            factor_f: None,
            factor_b: None,
        }.null_checked()
    }
}
