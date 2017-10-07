
extern crate fftw_sys as ffi;
extern crate num_traits;
extern crate num_complex;
#[macro_use]
extern crate lazy_static;

pub mod pair;
pub mod r2r;
pub mod aligned_vec;
pub mod plan;
mod util;

pub type FLAG = u32;
pub use ffi::SIGN;
pub use ffi::fftw_r2r_kind as R2R_KIND;

pub use ffi::fftw_complex as c64;
pub use ffi::fftwf_complex as c32;
pub use pair::*;
