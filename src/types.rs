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
bitflags! {
    #[derive(Default)]
    pub struct Flag: u32 {
        const Measure = 0;
        const DestroyInput = 1 ;
        const Unaligned = 1 << 1;
        const ConserveMemory = 1 << 2;
        const Exhausive = 1 << 3;
        const PreserveInput = 1 << 4;
        const Patient = 1 << 5;
        const Estimate = 1 << 6;
        const WisdowmOnly = 1 << 21;
    }
}
