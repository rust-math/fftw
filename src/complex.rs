
use ffi;
use num_traits::{One, Zero};
use num_extra::Exponential;
use std::ops::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, RustcEncodable, RustcDecodable)]
pub struct c64(ffi::fftw_complex);

impl Deref for c64 {
    type Target = ffi::fftw_complex;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for c64 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl c64 {
    pub fn re(&self) -> f64 { self[0] }
    pub fn im(&self) -> f64 { self[1] }
    pub fn new(re: f64, im: f64) -> Self { c64([re, im]) }
    pub fn abs(&self) -> f64 { (self.re() * self.re() + self.im() * self.im()).sqrt() }
}

impl Add for c64 {
    type Output = c64;
    fn add(self, other: Self) -> Self { c64([self.re() + other.re(), self.im() + other.im()]) }
}

impl AddAssign for c64 {
    fn add_assign(&mut self, other: Self) {
        self[0] += other[0];
        self[1] += other[1];
    }
}

impl Sub for c64 {
    type Output = c64;
    fn sub(self, other: Self) -> Self { c64([self.re() - other.re(), self.im() - other.im()]) }
}

impl SubAssign for c64 {
    fn sub_assign(&mut self, other: Self) {
        self[0] -= other[0];
        self[1] -= other[1];
    }
}

impl Neg for c64 {
    type Output = c64;
    fn neg(self) -> Self { c64([-self[0], -self[1]]) }
}

impl Mul for c64 {
    type Output = c64;
    fn mul(self, other: Self) -> Self {
        c64([self.re() * other.re() - self.im() * other.im(), self.re() * other.im() + self.im() * other.re()])
    }
}

impl MulAssign for c64 {
    fn mul_assign(&mut self, other: Self) {
        let a = self.re();
        let b = self.im();
        self[0] = a * other.re() - b * other.im();
        self[1] = a * other.im() + b * other.re();
    }
}

impl Div for c64 {
    type Output = c64;
    fn div(self, other: Self) -> Self {
        let a = other.re();
        let b = other.im();
        let c = self.re();
        let d = self.im();
        let abs = a * a + b * b;
        c64([(a * c + b * d) / abs, (a * d - b * c) / abs])
    }
}

impl Add<f64> for c64 {
    type Output = c64;
    fn add(self, other: f64) -> Self { c64([self.re() + other, self.im()]) }
}

impl AddAssign<f64> for c64 {
    fn add_assign(&mut self, rhs: f64) { self[0] += rhs; }
}

impl Mul<f64> for c64 {
    type Output = c64;
    fn mul(self, other: f64) -> Self { c64([self.re() * other, self.im() * other]) }
}

impl MulAssign<f64> for c64 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
    }
}

impl Div<f64> for c64 {
    type Output = c64;
    fn div(self, rhs: f64) -> Self { c64([self.re() / rhs, self.im() / rhs]) }
}

impl One for c64 {
    fn one() -> Self { c64([1.0, 0.0]) }
}

impl Zero for c64 {
    fn zero() -> Self { c64([0.0, 0.0]) }
    fn is_zero(&self) -> bool { self.re().is_zero() && self.im().is_zero() }
}

impl Exponential for c64 {
    fn exp(self) -> Self {
        let a = self.re();
        let b = self.im();
        let (s, c) = b.sin_cos();
        let ea = a.exp();
        c64([ea * c, ea * s])
    }
}
