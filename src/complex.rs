
use ffi;
use num_traits::{One, Zero};
use std::ops::{Add, Mul, Deref, DerefMut};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy)]
pub struct c64(ffi::fftw_complex);

impl Deref for c64 {
    type Target = ffi::fftw_complex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for c64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl c64 {
    pub fn re(&self) -> f64 {
        self[0]
    }
    pub fn im(&self) -> f64 {
        self[1]
    }
}

impl Add for c64 {
    type Output = c64;
    fn add(self, other: Self) -> Self {
        c64([self.re() + other.re(), self.im() + other.im()])
    }
}

impl Mul for c64 {
    type Output = c64;
    fn mul(self, other: Self) -> Self {
        c64([self.re() * other.re() - self.im() * other.im(),
             self.re() * other.im() + self.im() * other.re()])
    }
}

impl One for c64 {
    fn one() -> Self {
        c64([1.0, 0.0])
    }
}

impl Zero for c64 {
    fn zero() -> Self {
        c64([0.0, 0.0])
    }
    fn is_zero(&self) -> bool {
        self.re().is_zero() && self.im().is_zero()
    }
}
