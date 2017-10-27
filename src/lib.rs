
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate procedurals;

extern crate num_traits;
extern crate num_complex;
extern crate ndarray;
// XXX For ndarray_linalg::Scalar
// Will be removed if the following PR to num-complex is merged
// https://github.com/rust-num/num/pull/338
extern crate ndarray_linalg;

extern crate fftw_sys as ffi;

pub mod pair;
pub mod r2r;
pub mod r2c;
pub mod c2c;
pub mod array;
pub mod plan;
pub mod error;
pub mod traits;

pub type FLAG = u32;

pub use ffi::fftw_complex as c64;
pub use ffi::fftwf_complex as c32;

pub use c2c::*;
pub use pair::*;
pub use r2c::*;
pub use r2r::*;
pub use traits::*;

use std::sync::Mutex;
lazy_static! {
    pub static ref FFTW_MUTEX: Mutex<()> = Mutex::new(());
}
