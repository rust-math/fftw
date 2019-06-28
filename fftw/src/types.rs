//! Rusty types for manipulating FFTW

use bitflags::bitflags;
pub use ffi::fftw_complex as c64;
pub use ffi::fftwf_complex as c32;

/// Expose the kinds of real-to-real transformations
pub use ffi::fftw_r2r_kind as R2RKind;

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

bitflags! {
    /// Flags for creating plans and wisdom
    ///
    /// This will be the most important part for fast FFT.
    /// You should see the [Words of Wisdom] in the original document
    ///
    /// [Words of Wisdom]: http://www.fftw.org/fftw3_doc/Words-of-Wisdom_002dSaving-Plans.html
    #[derive(Default)]
    pub struct Flag: u32 {
        const MEASURE = 0;
        const DESTROYINPUT = 1 ;
        const UNALIGNED = 1 << 1;
        const CONSERVEMEMORY = 1 << 2;
        const EXHAUSIVE = 1 << 3;
        const PRESERVEINPUT = 1 << 4;
        const PATIENT = 1 << 5;
        const ESTIMATE = 1 << 6;
        const WISDOWMONLY = 1 << 21;
    }
}
