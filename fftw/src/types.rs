//! Rusty types for manipulating FFTW

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

/// Flags for creating plans and wisdom
///
/// This will be the most important part for fast FFT.
/// You should see the [Words of Wisdom] in the original document
///
/// [Words of Wisdom]: http://www.fftw.org/fftw3_doc/Words-of-Wisdom_002dSaving-Plans.html
bitflags! {
    #[derive(Default)]
    pub struct Flag: u32 {
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const Measure = 0;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const DestroyInput = 1 ;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const Unaligned = 1 << 1;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const ConserveMemory = 1 << 2;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const Exhausive = 1 << 3;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const PreserveInput = 1 << 4;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const Patient = 1 << 5;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const Estimate = 1 << 6;
        #[allow(non_upper_case_globals)] // FIXME for compatibility
        const WisdowmOnly = 1 << 21;
    }
}
