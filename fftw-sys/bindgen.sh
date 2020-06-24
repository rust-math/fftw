#!/bin/bash
set -uex

# Blacklist setting
#
# Types
# ------
# - "fftw.*_complex"
#   - Use `num_complex::Complex32` and `num_complex::Complex64`
# - "FILE"
#   - Use `libc::FILE` instead
# - "_.*"
#   - Remove unrelated
#
# Function
# ---------
# - "fftwl_.*"
#   - Disable `long double` interface
#
bindgen \
  --use-core \
  --with-derive-{default,eq,hash,ord} \
  --whitelist-type="^fftw.*" \
  --whitelist-var="^FFTW.*" \
  --whitelist-function="^fftw.*" \
  --blacklist-type="FILE" \
  --blacklist-type="_.*" \
  --blacklist-type="fftw.*_complex" \
  --blacklist-function="fftwl_.*" \
  --default-enum-style=rust \
  wrapper.h \
  > src/fftw.rs
