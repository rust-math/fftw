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
        {
            let _lock = $crate::FFTW_MUTEX.lock().expect("Cannot get lock");
            unsafe { $call }
        }
    }
}

pub mod array;
pub mod error;
pub mod plan;
pub mod types;
