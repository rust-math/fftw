rust-fftw3
===========
[![Build Status](https://travis-ci.org/termoshtt/rust-fftw3.svg?branch=master)](https://travis-ci.org/termoshtt/rust-fftw3)

FFTW binding for Rust

This repository includes three crates:

- [![Crate](http://meritbadge.herokuapp.com/fftw)](https://crates.io/crates/fftw)
  fftw: safe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-sys)](https://crates.io/crates/fftw-sys)
  fftw-sys: unsafe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-src)](https://crates.io/crates/fftw-src)
  fftw-src: source of FFTW


Usage
------

```
[dependencies]
fftw = 0.1.0
```

If you want use system's libfftw3

```
[dependencies.fftw-sys]
version = "0.1.0"
features = ["system"]
default-features = false
```

or

```
[dependencies]
fftw-sys = { version = "0.1.0", features = ["system"], default-features = false }
```

LICENSE
--------
FFTW is free software and distributed under GPLv2.
http://www.fftw.org/fftw3_doc/License-and-Copyright.html

This binding is licensed by GPLv3.
