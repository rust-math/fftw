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
- `system`: use system's libfftw3 (experimental)
    - You must install FFTW to your system before building this crate
    - For Linux system, e.g. Ubuntu or Debian, please run `apt install libfftw3-dev`
    - For macOS, please run `brew install fftw` by using [homebrew](https://github.com/Homebrew/brew)
    - For Windows, this feature is supported
- `intel-mkl` use Intel MKL backend through [intel-mkl-src](https://github.com/termoshtt/rust-intel-mkl)
    - Linux and Windows are supported

|Feature  | Linux | Windows | macOS |
|:--------|:-----:|:-------:|:-----:|
|source   |✔️      |✔️        |✔️      |
|system   |✔️      |-        |✔️      |
|intel-mkl|✔️      |✔️        |-      |

LICENSE
--------
See [LICENSE.md](./LICENSE.md)
