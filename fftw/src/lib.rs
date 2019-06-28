//! Rust binding of [FFTW]
//!
//! [FFTW]: http://www.fftw.org/
//!
//! Examples
//! ---------
//!
//! Complex-to-Complex
//!
//! ```
//! use fftw::array::AlignedVec;
//! use fftw::plan::*;
//! use fftw::types::*;
//! use std::f64::consts::PI;
//!
//! let n = 128;
//! let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
//! let mut a = AlignedVec::new(n);
//! let mut b = AlignedVec::new(n);
//! let k0 = 2.0 * PI / n as f64;
//! for i in 0..n {
//!     a[i] = c64::new((k0 * i as f64).cos(), 0.0);
//! }
//! plan.c2c(&mut a, &mut b).unwrap();
//! ```
//!
//! Complex-to-Real
//!
//! ```
//! use fftw::array::AlignedVec;
//! use fftw::plan::*;
//! use fftw::types::*;
//! use std::f64::consts::PI;
//!
//! let n = 128;
//! let mut c2r: C2RPlan64 = C2RPlan::aligned(&[n], Flag::MEASURE).unwrap();
//! let mut a = AlignedVec::new(n / 2 + 1);
//! let mut b = AlignedVec::new(n);
//! for i in 0..(n / 2 + 1) {
//!     a[i] = c64::new(1.0, 0.0);
//! }
//! c2r.c2r(&mut a, &mut b).unwrap();
//! ```

extern crate fftw_sys as ffi;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// Mutex for FFTW call.
    ///
    /// This mutex is necessary because most of calls in FFTW are not thread-safe.
    /// See the [original document](http://www.fftw.org/fftw3_doc/Thread-safety.html) for detail
    pub static ref FFTW_MUTEX: Mutex<()> = Mutex::new(());
}

/// Exclusive call of FFTW interface.
macro_rules! excall {
    ($call:expr) => {{
        let _lock = $crate::FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe { $call }
    }};
} // excall!

pub mod array;
pub mod error;
pub mod plan;
pub mod types;
