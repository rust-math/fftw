
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate procedurals;

extern crate num_traits;
extern crate num_complex;
extern crate ndarray;

extern crate fftw_sys as ffi;

pub mod pair;
pub mod r2r;
pub mod c2c;
pub mod aligned_vec;
pub mod plan;
pub mod error;

pub use ffi::SIGN;
pub use ffi::fftw_r2r_kind as R2R_KIND;
pub type FLAG = u32;

pub use ffi::fftw_complex as c64;
pub use ffi::fftwf_complex as c32;

pub use c2c::*;
pub use pair::*;
pub use r2r::*;

use std::sync::Mutex;
lazy_static! {
    pub static ref FFTW_MUTEX: Mutex<()> = Mutex::new(());
}
