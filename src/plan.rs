
use ffi;
use super::enums::*;
use super::raw_vec::RawVec;
use super::util::FFTW_MUTEX;
use super::{c32, c64};

use std::marker::PhantomData;

pub struct Plan<A, B> {
    plan: RawPlan,
    phantom: PhantomData<(A, B)>,
}

impl<A, B> Plan<A, B> {
    /// this function modifys the array referred in plan creation
    pub unsafe fn execute(&self) { self.plan.execute() }
}

impl<T: R2RPlanCreate> Plan<T, T> {
    pub fn r2r_1d(n: usize,
                  mut in_: &mut RawVec<T>,
                  mut out: &mut RawVec<T>,
                  kind: R2R_KIND,
                  flag: FLAG)
                  -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe { T::r2r_1d(n, &mut in_, &mut out, kind, flag) };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }
}

impl<T: C2CPlanCreate> Plan<T, T> {
    pub fn c2c_1d(n: usize,
                  mut in_: &mut RawVec<T>,
                  mut out: &mut RawVec<T>,
                  sign: SIGN,
                  flag: FLAG)
                  -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe { T::c2c_1d(n, &mut in_, &mut out, sign, flag) };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }
}

impl<C, R> Plan<C, R>
    where (C, R): C2RPlanCreate<Real = R, Complex = C>
{
    pub fn c2r_1d(n: usize, in_: &mut RawVec<C>, out: &mut RawVec<R>, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe { <(C, R)>::c2r_1d(n, in_, out, flag) };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }
}

impl<R, C> Plan<R, C>
    where (C, R): C2RPlanCreate<Real = R, Complex = C>
{
    pub fn r2c_1d(n: usize, in_: &mut RawVec<R>, out: &mut RawVec<C>, flag: FLAG) -> Self {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        let plan = unsafe { <(C, R)>::r2c_1d(n, in_, out, flag) };
        drop(lock);
        Plan {
            plan: plan,
            phantom: PhantomData,
        }
    }
}

pub enum RawPlan {
    _64(ffi::fftw_plan),
    _32(ffi::fftwf_plan),
}

impl RawPlan {
    pub unsafe fn execute(&self) {
        match *self {
            RawPlan::_64(p) => ffi::fftw_execute(p),
            RawPlan::_32(p) => ffi::fftwf_execute(p),
        }
    }
}

impl Drop for RawPlan {
    fn drop(&mut self) {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe {
            match *self {
                RawPlan::_64(p) => ffi::fftw_destroy_plan(p),
                RawPlan::_32(p) => ffi::fftwf_destroy_plan(p),
            }
        }
        drop(lock);
    }
}

pub trait R2RPlanCreate: Sized {
    unsafe fn r2r_1d(n: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     R2R_KIND,
                     FLAG)
                     -> RawPlan;
    unsafe fn r2r_2d(n0: usize,
                     n1: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     R2R_KIND,
                     R2R_KIND,
                     FLAG)
                     -> RawPlan;
    unsafe fn r2r_3d(n0: usize,
                     n1: usize,
                     n2: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     R2R_KIND,
                     R2R_KIND,
                     R2R_KIND,
                     FLAG)
                     -> RawPlan;
}
pub trait C2CPlanCreate: Sized {
    unsafe fn c2c_1d(n: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     SIGN,
                     FLAG)
                     -> RawPlan;
    unsafe fn c2c_2d(n0: usize,
                     n1: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     SIGN,
                     FLAG)
                     -> RawPlan;
    unsafe fn c2c_3d(n0: usize,
                     n1: usize,
                     n2: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     SIGN,
                     FLAG)
                     -> RawPlan;
}

pub trait C2RPlanCreate {
    type Real: Sized;
    type Complex: Sized;
    unsafe fn r2c_1d(n: usize,
                     in_: &mut RawVec<Self::Real>,
                     out: &mut RawVec<Self::Complex>,
                     FLAG)
                     -> RawPlan;
    unsafe fn c2r_1d(n: usize,
                     in_: &mut RawVec<Self::Complex>,
                     out: &mut RawVec<Self::Real>,
                     FLAG)
                     -> RawPlan;
    unsafe fn r2c_2d(n0: usize,
                     n1: usize,
                     in_: &mut RawVec<Self::Real>,
                     out: &mut RawVec<Self::Complex>,
                     FLAG)
                     -> RawPlan;
    unsafe fn c2r_2d(n0: usize,
                     n1: usize,
                     in_: &mut RawVec<Self::Complex>,
                     out: &mut RawVec<Self::Real>,
                     FLAG)
                     -> RawPlan;
    unsafe fn r2c_3d(n0: usize,
                     n1: usize,
                     n2: usize,
                     in_: &mut RawVec<Self::Real>,
                     out: &mut RawVec<Self::Complex>,
                     FLAG)
                     -> RawPlan;
    unsafe fn c2r_3d(n0: usize,
                     n1: usize,
                     n2: usize,
                     in_: &mut RawVec<Self::Complex>,
                     out: &mut RawVec<Self::Real>,
                     FLAG)
                     -> RawPlan;
}

macro_rules! impl_plan_create {
    ($bit:ident, $float:ty, $complex:ty,
     $r2r_1d:ident, $r2c_1d:ident, $c2r_1d:ident, $c2c_1d:ident,
     $r2r_2d:ident, $r2c_2d:ident, $c2r_2d:ident, $c2c_2d:ident,
     $r2r_3d:ident, $r2c_3d:ident, $c2r_3d:ident, $c2c_3d:ident) => {

impl R2RPlanCreate for $float {
    unsafe fn r2r_1d(n: usize, in_: &mut RawVec<Self>, out: &mut RawVec<Self>, kind: R2R_KIND, flag: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2r_1d(n as i32, in_.as_mut_ptr(), out.as_mut_ptr(), kind, flag as u32))
    }
    unsafe fn r2r_2d(n0: usize, n1: usize, in_: &mut RawVec<Self>, out: &mut RawVec<Self>, k0: R2R_KIND, k1: R2R_KIND, flag: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2r_2d(n0 as i32, n1 as i32, in_.as_mut_ptr(), out.as_mut_ptr(), k0, k1, flag as u32))
    }
    unsafe fn r2r_3d(n0: usize, n1: usize, n2: usize, in_: &mut RawVec<Self>, out: &mut RawVec<Self>, k0: R2R_KIND, k1: R2R_KIND, k2: R2R_KIND, flag: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2r_3d(n0 as i32, n1 as i32, n2 as i32, in_.as_mut_ptr(), out.as_mut_ptr(), k0, k1, k2, flag as u32))
    }
}

impl C2CPlanCreate for $complex {
    unsafe fn c2c_1d(n: usize, i: &mut RawVec<Self>, o: &mut RawVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32))
    }
    unsafe fn c2c_2d(n0: usize, n1: usize, i: &mut RawVec<Self>, o: &mut RawVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2c_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32))
    }
    unsafe fn c2c_3d(n0: usize, n1: usize, n2: usize, i: &mut RawVec<Self>, o: &mut RawVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2c_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32))
    }
}

impl C2RPlanCreate for ($complex, $float) {
    type Real = $float;
    type Complex = $complex;
    unsafe fn r2c_1d(n: usize, i: &mut RawVec<Self::Real>, o: &mut RawVec<Self::Complex>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn c2r_1d(n: usize, i: &mut RawVec<Self::Complex>, o: &mut RawVec<Self::Real>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2r_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn r2c_2d(n0: usize, n1: usize, i: &mut RawVec<Self::Real>, o: &mut RawVec<Self::Complex>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2c_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn c2r_2d(n0: usize, n1: usize, i: &mut RawVec<Self::Complex>, o: &mut RawVec<Self::Real>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2r_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn r2c_3d(n0: usize, n1: usize, n2: usize, i: &mut RawVec<Self::Real>, o: &mut RawVec<Self::Complex>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2c_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
    unsafe fn c2r_3d(n0: usize, n1: usize, n2: usize, i: &mut RawVec<Self::Complex>, o: &mut RawVec<Self::Real>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2r_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32))
    }
}

}} // impl_plan_create

impl_plan_create!(_64,
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
                  fftw_plan_dft_3d);
impl_plan_create!(_32,
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
                  fftwf_plan_dft_3d);
