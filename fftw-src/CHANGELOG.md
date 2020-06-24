Changelog for fftw-src crate
=============================

Unreleased
----------

0.3.2 - 2020-06-24
-------------------

### Add

- Bundle generated source code of FFTW-3.3.8 https://github.com/rust-math/fftw/pull/88
  - build in OUT_DIR https://github.com/rust-math/fftw/pull/93

### Change
- failure -> anyhow https://github.com/rust-math/fftw/pull/91

### Delete
- Drop reqwest, md5 crates https://github.com/rust-math/fftw/pull/91

0.3.1 - 2020-05-25
-------------------

### Change
- Fix current path for macOS https://github.com/rust-math/fftw/pull/75
- Switch to Rust 2018 https://github.com/rust-math/fftw/pull/69

0.3.0 - 2019-05-02
===================

### Add
- Support Windows https://github.com/rust-math/fftw/pull/66
