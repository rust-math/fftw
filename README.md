rust-fftw3
===========
![Rust](https://github.com/rust-math/fftw/workflows/Rust/badge.svg)

Rust bindings for the [FFTW C-library](http://www.fftw.org/) for computing discrete Fourier transforms, as well as discrete cosine and sine transforms.

This repository includes three crates:

- [![Crate](http://meritbadge.herokuapp.com/fftw)](https://crates.io/crates/fftw)
  [![docs.rs](https://docs.rs/fftw/badge.svg)](https://docs.rs/fftw)
  `fftw`: A safe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-sys)](https://crates.io/crates/fftw-sys)
  [![docs.rs](https://docs.rs/fftw-sys/badge.svg)](https://docs.rs/fftw-sys)
  `fftw-sys`: An unsafe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-src)](https://crates.io/crates/fftw-src)
  [![docs.rs](https://docs.rs/fftw-src/badge.svg)](https://docs.rs/fftw-src)
  `fftw-src`: A crate for downloading and compiling the FFTW library


Feature flags
--------------

- `source`: Download and compile FFTW (default)
    - (Linux, macOS) Needs a C-compiler and the `make` build tool to compile the FFTW library
    - (Windows) Downloads a precompiled binary from the [FFTW website](http://www.fftw.org/install/windows.html)
- `system`: Use the system's libfftw3 (experimental)
    - You must install FFTW before building this crate
    - For Linux systems, please install FFTW using your package manager, e.g. in Debian or Ubuntu run `apt install libfftw3-dev`
    - For macOS, please run `brew install fftw` by using [homebrew](https://github.com/Homebrew/brew)
    - This feature is unsupported on Windows
- `intel-mkl` Use Intel MKL backend through [intel-mkl-src](https://github.com/termoshtt/rust-intel-mkl)
    - Only Linux and Windows are supported

|Feature  | Linux | Windows | macOS | iOS(arm64) | Android(armv7a/arm64) |
|:--------|:-----:|:-------:|:-----:|:----------:|:---------------------:|
|source   |✔️      |✔️        |✔️      |✔️           |✔️                      |
|system   |✔️      |-        |✔️      |-           |-                      |
|intel-mkl|✔️      |✔️        |-      |-           |-                      |

LICENSE
--------
See [LICENSE.md](./LICENSE.md)
