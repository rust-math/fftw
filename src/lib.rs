#![allow(non_camel_case_types)]

extern crate fftw_sys as ffi;
extern crate num_traits;
#[macro_use]
extern crate lazy_static;

mod pair;
mod complex;
mod r2r;
mod raw_vec;
mod plan;
mod enums;
mod util;

pub use pair::*;
pub use complex::c64;
pub use enums::*;
pub use raw_vec::RawVec;
