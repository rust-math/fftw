#![allow(non_camel_case_types)]

#[cfg(feature = "source")]
extern crate fftw_src as ffi;

#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src as ffi;

use libc::FILE;
pub use num_complex::Complex32 as fftwf_complex;
pub use num_complex::Complex64 as fftw_complex;

#[cfg_attr(feature = "system", link(name = "fftw3"))]
extern "C" {}
#[cfg_attr(feature = "system", link(name = "fftw3f"))]
extern "C" {}

include!("fftw.rs");
