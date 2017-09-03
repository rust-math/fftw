rust-fftw3
===========
[![Build Status](https://travis-ci.org/termoshtt/rust-fftw3.svg?branch=master)](https://travis-ci.org/termoshtt/rust-fftw3)

FFTW binding for Rust

This repository includes three crates:

- [![Crate](http://meritbadge.herokuapp.com/fftw)](https://crates.io/crates/fftw)
  [![docs](https://img.shields.io/badge/docs-gh--pages-blue.svg)](https://termoshtt.github.io/rust-fftw3/fftw)
  fftw: safe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-sys)](https://crates.io/crates/fftw-sys)
  [![docs](https://img.shields.io/badge/docs-gh--pages-blue.svg)](https://termoshtt.github.io/rust-fftw3/fftw_sys)
  fftw-sys: unsafe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-src)](https://crates.io/crates/fftw-src)
  [![docs](https://img.shields.io/badge/docs-gh--pages-blue.svg)](https://termoshtt.github.io/rust-fftw3/fftw_src)
  fftw-src: source of FFTW


Usage
------

```
[dependencies]
fftw = 0.2.0
```

- use system's libfftw3

```
[dependencies]
fftw-sys = { version = "0.2.0", features = ["system"], default-features = false }
```

- use Intel MKL backend through [intel-mkl-src crate](https://github.com/termoshtt/rust-intel-mkl)

```
[dependencies]
fftw-sys = { version = "0.2.0", features = ["intel-mkl"], default-features = false }
```

LICENSE
--------

The wrapper codes in this crate are licensed by GPLv3 (see [LICENSE](LICENSE)),
and the backends are redistributed under following licenses:

- [FFTW](http://www.fftw.org/) is free software and distributed under GPLv2 ([License and Copyright](http://www.fftw.org/fftw3_doc/License-and-Copyright.html))
- [Intel MKL](https://software.intel.com/en-us/mkl) is distributed under the [Intel Simplified Software License for Intel(R) Math Kernel Library](https://github.com/termoshtt/rust-intel-mkl/blob/master/mkl_lib/license.txt)
