Changelog for fftw crate
=========================

Unreleased
----------

0.6.1 - 2020-06-20
-------------------

### Change
- Add alignment and size check for C2C, R2R https://github.com/rust-math/fftw/pull/84
- Switch to Rust 2018 https://github.com/rust-math/fftw/pull/69
- Rename flags for FFTW plan e.g. `Flag::Measure` -> `Flag::MEASURE` https://github.com/rust-math/fftw/pull/70

### Maintenance
- Switch CI from Azure Pipeline to GitHub Action https://github.com/rust-math/fftw/pull/80
- Change License GPLv2 -> GPLv2 or Later https://github.com/rust-math/fftw/pull/74

0.6.0 - 2019-05-02
===================

### Add
- Real-to-Real transform https://github.com/rust-math/fftw/pull/67

### Change
- fftw-sys 0.5.0
- Change to dual License GPLv2 or MIT + Intel Simplified Software License (for intel-mkl feature)
