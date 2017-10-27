use super::FLAG;
use super::array::*;
use super::error::*;
use super::pair::*;
use super::plan::*;
use super::traits::*;

use ffi;
pub use ffi::fftw_r2r_kind as R2R_KIND;

use ndarray::*;
use ndarray_linalg::Scalar;

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

impl<T: FFTWReal> ToPair<T, T> for R2R1D {
    type Dim = Ix1;
    fn to_pair(&self) -> Result<Pair<T, T, Ix1>> {
        let mut a = AlignedVec::new(self.n);
        let mut b = AlignedVec::new(self.n);
        let forward = unsafe { T::r2r_1d(self.n, &mut a, &mut b, forward(self.kind), self.flag) };
        let backward = unsafe { T::r2r_1d(self.n, &mut b, &mut a, backward(self.kind), self.flag) };
        Pair {
            a: AlignedArray::from_vec(a),
            b: AlignedArray::from_vec(b),
            forward: Plan::with_factor(forward, Scalar::from_f64(1.0 / self.n as f64)),
            backward: Plan::new(backward),
        }.null_checked()
    }
}
