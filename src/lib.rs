//! Rust binding of [FFTW]
//!
//! [FFTW]: http://www.fftw.org/

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate procedurals;

extern crate fftw_sys as ffi;

extern crate ndarray;
extern crate num_complex;
extern crate num_traits;

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
