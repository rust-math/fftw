#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate procedurals;

extern crate ndarray;
extern crate fftw_sys as ffi;

use std::sync::Mutex;
lazy_static! {
    pub static ref FFTW_MUTEX: Mutex<()> = Mutex::new(());
}

macro_rules! excall {
    ($call:expr) => {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe { $call }
    }
}

pub mod array;
pub mod error;
pub mod nae;

pub use ffi::fftw_complex as c64;
pub use ffi::fftwf_complex as c32;

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
        use Flag::*;
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
