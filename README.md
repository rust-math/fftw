rust-fftw3
===========
![Rust](https://github.com/rust-math/fftw/workflows/Rust/badge.svg)

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
See [LICENSE.md](./LICENSE.md)
