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

If you want use system's libfftw3

```
[dependencies.fftw-sys]
version = "0.2.0"
features = ["system"]
default-features = false
```

or

```
[dependencies]
fftw-sys = { version = "0.2.0", features = ["system"], default-features = false }
```

LICENSE
--------
FFTW is free software and distributed under GPLv2.
http://www.fftw.org/fftw3_doc/License-and-Copyright.html

This binding is licensed by GPLv3.
