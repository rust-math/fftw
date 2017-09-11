rust-fftw3 [![Build Status](https://travis-ci.org/termoshtt/rust-fftw3.svg?branch=master)](https://travis-ci.org/termoshtt/rust-fftw3)
===========

FFTW binding for Rust

This repository includes three crates:

- [![Crate](http://meritbadge.herokuapp.com/fftw)](https://crates.io/crates/fftw)
  [![docs.rs](https://docs.rs/fftw/badge.svg)](https://docs.rs/fftw)
  fftw: safe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-sys)](https://crates.io/crates/fftw-sys)
  [![docs.rs](https://docs.rs/fftw-sys/badge.svg)](https://docs.rs/fftw-sys)
  fftw-sys: unsafe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-src)](https://crates.io/crates/fftw-src)
  [![docs.rs](https://docs.rs/fftw-src/badge.svg)](https://docs.rs/fftw-src)
  fftw-src: source of FFTW


Feature flags
--------------

- `source`: download and complie FFTW (defualt)
    - You need `curl` to download the source, and C-compiler and `make` to compile FFTW
- `system`: use system's libfftw3 (experimental)
    - You must install FFTW to your system before building this crate
- `intel-mkl` use Intel MKL backend through [intel-mkl-src](https://github.com/termoshtt/rust-intel-mkl) (experimental)
    - You need `curl` to download

LICENSE
--------

The codes in this crate are licensed by MIT-License (see [LICENSE](LICENSE)),
and the backends are redistributed under following licenses:

- [FFTW](http://www.fftw.org/) is free software and distributed under GPLv2 ([License and Copyright](http://www.fftw.org/fftw3_doc/License-and-Copyright.html))
- [Intel MKL](https://software.intel.com/en-us/mkl) is distributed under the [Intel Simplified Software License for Intel(R) Math Kernel Library](https://github.com/termoshtt/rust-intel-mkl/blob/master/mkl_lib/license.txt)
