use super::{FFTW_MUTEX, FLAG, R2R_KIND, SIGN, c32, c64};
use super::array::AlignedVec;
use super::error::*;
use ffi;

use ndarray_linalg::Scalar;
use std::os::raw::c_void;
use std::ptr::null;

pub struct Plan<T: Scalar> {
    p: RawPlan,
    factor: Option<T::Real>,
}

impl<T: Scalar> Plan<T> {
    pub fn new(p: RawPlan) -> Self {
        Self { p, factor: None }
    }

    pub fn with_factor(p: RawPlan, f: T::Real) -> Self {
        Self { p, factor: Some(f) }
    }

    pub unsafe fn execute(&self) {
        self.p.execute()
    }

    fn get_factor(&self) -> Option<&T::Real> {
        self.factor.as_ref()
    }

    pub fn check_null(&self) -> Result<()> {
        if self.p.is_null() {
            Err(InvalidPlanError::new().into())
        } else {
            Ok(())
        }
    }

    pub fn normalize(&self, array: &mut [T]) {
        if let Some(n) = self.get_factor() {
            for val in array.iter_mut() {
                *val = val.mul_real(*n);
            }
        }
    }
}

pub enum RawPlan {
    _64(ffi::fftw_plan),
    _32(ffi::fftwf_plan),
}

impl RawPlan {
    /// Execute FFT saved in the plan
    ///
    /// This is unsafe because rewrite the array saved in the plan.
    pub unsafe fn execute(&self) {
        if self.is_null() {
            panic!("Plan is NULL");
        }
        match *self {
            RawPlan::_64(p) => ffi::fftw_execute(p),
            RawPlan::_32(p) => ffi::fftwf_execute(p),
        }
    }

    /// Check if the plan is NULL
    pub fn is_null(&self) -> bool {
        let p = match *self {
            RawPlan::_64(p) => p as *const c_void,
            RawPlan::_32(p) => p as *const c_void,
        };
        p == null()
    }
}

impl Drop for RawPlan {
    fn drop(&mut self) {
        if self.is_null() {
            // TODO warning
            return;
        }
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe {
            match *self {
                RawPlan::_64(p) => ffi::fftw_destroy_plan(p),
                RawPlan::_32(p) => ffi::fftwf_destroy_plan(p),
            }
        }
    }
}

pub trait R2R: Sized {
    unsafe fn r2r_1d(n: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, R2R_KIND, FLAG) -> RawPlan;
    unsafe fn r2r_2d(
        n0: usize,
        n1: usize,
        in_: &mut AlignedVec<Self>,
        out: &mut AlignedVec<Self>,
        R2R_KIND,
        R2R_KIND,
        FLAG,
    ) -> RawPlan;
    unsafe fn r2r_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        in_: &mut AlignedVec<Self>,
        out: &mut AlignedVec<Self>,
        R2R_KIND,
        R2R_KIND,
        R2R_KIND,
        FLAG,
    ) -> RawPlan;
}
pub trait C2C: Sized {
    unsafe fn c2c_1d(n: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, SIGN, FLAG) -> RawPlan;
    unsafe fn c2c_2d(
        n0: usize,
        n1: usize,
        in_: &mut AlignedVec<Self>,
        out: &mut AlignedVec<Self>,
        SIGN,
        FLAG,
    ) -> RawPlan;
    unsafe fn c2c_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        in_: &mut AlignedVec<Self>,
        out: &mut AlignedVec<Self>,
        SIGN,
        FLAG,
    ) -> RawPlan;
}

pub trait R2C {
    type Real: Sized;
    type Complex: Sized;
    unsafe fn r2c_1d(n: usize, in_: &mut AlignedVec<Self::Real>, out: &mut AlignedVec<Self::Complex>, FLAG) -> RawPlan;
    unsafe fn c2r_1d(n: usize, in_: &mut AlignedVec<Self::Complex>, out: &mut AlignedVec<Self::Real>, FLAG) -> RawPlan;
    unsafe fn r2c_2d(
        n0: usize,
        n1: usize,
        in_: &mut AlignedVec<Self::Real>,
        out: &mut AlignedVec<Self::Complex>,
        FLAG,
    ) -> RawPlan;
    unsafe fn c2r_2d(
        n0: usize,
        n1: usize,
        in_: &mut AlignedVec<Self::Complex>,
        out: &mut AlignedVec<Self::Real>,
        FLAG,
    ) -> RawPlan;
    unsafe fn r2c_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        in_: &mut AlignedVec<Self::Real>,
        out: &mut AlignedVec<Self::Complex>,
        FLAG,
    ) -> RawPlan;
    unsafe fn c2r_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        in_: &mut AlignedVec<Self::Complex>,
        out: &mut AlignedVec<Self::Real>,
        FLAG,
    ) -> RawPlan;
}

macro_rules! impl_plan_create {
    ($bit:ident, $float:ty, $complex:ty,
     $r2r_1d:ident, $r2c_1d:ident, $c2r_1d:ident, $c2c_1d:ident,
     $r2r_2d:ident, $r2c_2d:ident, $c2r_2d:ident, $c2c_2d:ident,
     $r2r_3d:ident, $r2c_3d:ident, $c2r_3d:ident, $c2c_3d:ident) => {

impl R2R for $float {
    unsafe fn r2r_1d(n: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, kind: R2R_KIND, flag: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$r2r_1d(n as i32, in_.as_mut_ptr(), out.as_mut_ptr(), kind, flag as u32))
    }
    unsafe fn r2r_2d(n0: usize, n1: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, k0: R2R_KIND, k1: R2R_KIND, flag: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$r2r_2d(n0 as i32, n1 as i32, in_.as_mut_ptr(), out.as_mut_ptr(), k0, k1, flag as u32))
    }
    unsafe fn r2r_3d(n0: usize, n1: usize, n2: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, k0: R2R_KIND, k1: R2R_KIND, k2: R2R_KIND, flag: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$r2r_3d(n0 as i32, n1 as i32, n2 as i32, in_.as_mut_ptr(), out.as_mut_ptr(), k0, k1, k2, flag as u32))
    }
}

impl C2C for $complex {
    unsafe fn c2c_1d(n: usize, i: &mut AlignedVec<Self>, o: &mut AlignedVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$c2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32))
    }
    unsafe fn c2c_2d(n0: usize, n1: usize, i: &mut AlignedVec<Self>, o: &mut AlignedVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$c2c_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32))
    }
    unsafe fn c2c_3d(n0: usize, n1: usize, n2: usize, i: &mut AlignedVec<Self>, o: &mut AlignedVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$c2c_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32))
    }
}

impl R2C for ($float, $complex) {
    type Real = $float;
    type Complex = $complex;
    unsafe fn r2c_1d(n: usize, i: &mut AlignedVec<Self::Real>, o: &mut AlignedVec<Self::Complex>, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$r2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn c2r_1d(n: usize, i: &mut AlignedVec<Self::Complex>, o: &mut AlignedVec<Self::Real>, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$c2r_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn r2c_2d(n0: usize, n1: usize, i: &mut AlignedVec<Self::Real>, o: &mut AlignedVec<Self::Complex>, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$r2c_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn c2r_2d(n0: usize, n1: usize, i: &mut AlignedVec<Self::Complex>, o: &mut AlignedVec<Self::Real>, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$c2r_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn r2c_3d(n0: usize, n1: usize, n2: usize, i: &mut AlignedVec<Self::Real>, o: &mut AlignedVec<Self::Complex>, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$r2c_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn c2r_3d(n0: usize, n1: usize, n2: usize, i: &mut AlignedVec<Self::Complex>, o: &mut AlignedVec<Self::Real>, f: FLAG) -> RawPlan {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        RawPlan::$bit(ffi::$c2r_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
}

}} // impl_plan_create

impl_plan_create!(
    _64,
    f64,
    c64,
    fftw_plan_r2r_1d,
    fftw_plan_dft_r2c_1d,
    fftw_plan_dft_c2r_1d,
    fftw_plan_dft_1d,
    fftw_plan_r2r_2d,
    fftw_plan_dft_r2c_2d,
    fftw_plan_dft_c2r_2d,
    fftw_plan_dft_2d,
    fftw_plan_r2r_3d,
    fftw_plan_dft_r2c_3d,
    fftw_plan_dft_c2r_3d,
    fftw_plan_dft_3d
);
impl_plan_create!(
    _32,
    f32,
    c32,
    fftwf_plan_r2r_1d,
    fftwf_plan_dft_r2c_1d,
    fftwf_plan_dft_c2r_1d,
    fftwf_plan_dft_1d,
    fftwf_plan_r2r_2d,
    fftwf_plan_dft_r2c_2d,
    fftwf_plan_dft_c2r_2d,
    fftwf_plan_dft_2d,
    fftwf_plan_r2r_3d,
    fftwf_plan_dft_r2c_3d,
    fftwf_plan_dft_c2r_3d,
    fftwf_plan_dft_3d
);
