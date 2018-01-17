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
    pub static ref FFTW_MUTEX: Mutex<()> = Mutex::new(());
}

/// Exclusive call of FFTW interface.
macro_rules! excall {
    ($call:expr) => {
    {
        let _lock = $crate::FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe { $call }
    }
}} // excall!

pub mod array;
pub mod error;
pub mod types;
pub mod fftw;
pub mod plan;
