//! Rusty types for manipulating FFTW

use ffi;

pub use ffi::fftw_complex as c64;
pub use ffi::fftwf_complex as c32;

/// Expose the kinds of real-to-real transformations
pub type R2RKind = ffi::fftw_r2r_kind;

/// Direction of Complex-to-Complex transformation
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Sign {
    Forward = -1,
    Backward = 1,
}

impl ::std::ops::Neg for Sign {
    type Output = Sign;
    fn neg(self) -> Self::Output {
        match self {
            Sign::Forward => Sign::Backward,
            Sign::Backward => Sign::Forward,
        }
    }
}

/// Flags for creating plans and wisdom
///
/// This will be the most important part for fast FFT.
/// You should see the [Words of Wisdom] in the original document
///
/// [Words of Wisdom]: http://www.fftw.org/fftw3_doc/Words-of-Wisdom_002dSaving-Plans.html
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Flag {
    Measure,
    DestroyInput,
    Unaligned,
    ConserveMemory,
    Exhausive,
    PreserveInput,
    Patient,
    Estimate,
    WisdowmOnly,
    Mixed(u32),
}

impl Into<u32> for Flag {
    fn into(self) -> u32 {
        use self::Flag::*;
        match self {
            Measure => 0,
            DestroyInput => 1 << 0,
            Unaligned => 1 << 1,
            ConserveMemory => 1 << 2,
            Exhausive => 1 << 3,
            PreserveInput => 1 << 4,
            Patient => 1 << 5,
            Estimate => 1 << 6,
            WisdowmOnly => 1 << 21,
            Mixed(u) => u,
        }
    }
}

impl ::std::ops::BitOr for Flag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        let lhs: u32 = self.into();
        let rhs: u32 = rhs.into();
        Flag::Mixed(lhs | rhs)
    }
}
