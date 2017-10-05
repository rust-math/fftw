
extern crate fftw_sys as ffi;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;
extern crate num_traits;
extern crate num_complex;
extern crate ndarray;

pub mod pair;
pub mod r2r;
pub mod aligned_vec;
pub mod plan;
pub mod enums;
mod util;

pub use enums::*;
pub use ffi::fftw_complex as c64;
pub use ffi::fftwf_complex as c32;
pub use pair::*;
