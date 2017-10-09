use super::*;
use aligned_vec::AlignedAllocable;
use plan::*;

use ndarray_linalg::Scalar;
use num_traits::*;

pub trait FFTWReal: Scalar + R2R + AlignedAllocable + Zero {}
pub trait FFTWComplex: Scalar + C2C + AlignedAllocable + Zero {}

impl FFTWReal for f32 {}
impl FFTWReal for f64 {}
impl FFTWComplex for c32 {}
impl FFTWComplex for c64 {}
