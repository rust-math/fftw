rust-fftw3
===========
[![Build Status](https://dev.azure.com/rust-math/rust-fftw3/_apis/build/status/rust-math.rust-fftw3?branchName=master)](https://dev.azure.com/rust-math/rust-fftw3/_build/latest?definitionId=2&branchName=master)

FFTW binding for Rust

This repository includes three crates:

- [![Crate](http://meritbadge.herokuapp.com/fftw)](https://crates.io/crates/fftw)
  [![docs.rs](https://docs.rs/fftw/badge.svg)](https://docs.rs/fftw)
  `fftw`: safe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-sys)](https://crates.io/crates/fftw-sys)
  [![docs.rs](https://docs.rs/fftw-sys/badge.svg)](https://docs.rs/fftw-sys)
  `fftw-sys`: unsafe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-src)](https://crates.io/crates/fftw-src)
  [![docs.rs](https://docs.rs/fftw-src/badge.svg)](https://docs.rs/fftw-src)
  `fftw-src`: source of FFTW


Feature flags
--------------

- `source`: download and compile FFTW (default)
    - (Linux, macOS) Needs C-compiler and `make` command to compile FFTW
    - (Windows) Download precompiled binary from [FFTW page](http://www.fftw.org/install/windows.html)
- `system`: use system's libfftw3 (experimental, Linux only)
    - You must install FFTW to your system before building this crate
- `intel-mkl` use Intel MKL backend through [intel-mkl-src](https://github.com/termoshtt/rust-intel-mkl)
    - Linux, macOS, and Windows are supported
    
LICENSE
--------

The codes in this crate are licensed by MIT-License (see [LICENSE](LICENSE)),
and the backends are redistributed under following licenses:

- [FFTW](http://www.fftw.org/) is free software and distributed under GPLv2 ([License and Copyright](http://www.fftw.org/fftw3_doc/License-and-Copyright.html))
- [Intel MKL](https://software.intel.com/en-us/mkl) is distributed under the [Intel Simplified Software License for Intel(R) Math Kernel Library](https://github.com/termoshtt/rust-intel-mkl/blob/master/mkl_lib/license.txt)
