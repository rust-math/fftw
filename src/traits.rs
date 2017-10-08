use super::*;
use aligned_vec::AlignedAllocable;
use plan::*;

use num_traits::*;

pub trait FFTWReal: R2R + AlignedAllocable + Zero {}
pub trait FFTWComplex: C2C + AlignedAllocable + Zero {}

impl FFTWReal for f32 {}
impl FFTWReal for f64 {}
impl FFTWComplex for c32 {}
impl FFTWComplex for c64 {}
