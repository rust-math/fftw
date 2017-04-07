
use ffi;
use super::enums::*;
use super::raw_vec::RawVec;
use super::util::FFTW_MUTEX;

use num_complex::Complex64 as c64;
use num_complex::Complex32 as c32;
use std::marker::PhantomData;

pub struct Plan<A, B> {
    plan: ffi::fftw_plan,
    phantom: PhantomData<(A, B)>,
}

impl<A, B> Plan<A, B> {
    /// this function modifys the array referred in plan creation
    pub unsafe fn execute(&self) { ffi::fftw_execute(self.plan); }
}

impl<A, B> Drop for Plan<A, B> {
    fn drop(&mut self) {
        let lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe {
            ffi::fftw_destroy_plan(self.plan);
        }
        drop(lock);
    }
}

impl<T: R2RPlanCreate> Plan<T, T> {
    pub fn r2r_1d(n: usize, mut in_: &mut RawVec<T>, mut out: &mut RawVec<T>, kind: R2R_KIND, flag: FLAG) -> Self {
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
    pub fn c2c_1d(n: usize, mut in_: &mut RawVec<T>, mut out: &mut RawVec<T>, sign: SIGN, flag: FLAG) -> Self {
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

pub trait R2RPlanCreate: Sized {
    unsafe fn r2r_1d(n: usize, in_: &mut RawVec<Self>, out: &mut RawVec<Self>, R2R_KIND, FLAG) -> ffi::fftw_plan;
}

impl R2RPlanCreate for f64 {
    unsafe fn r2r_1d(n: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     kind: R2R_KIND,
                     flag: FLAG)
                     -> ffi::fftw_plan {
        ffi::fftw_plan_r2r_1d(n as i32,
                              in_.as_mut_ptr(),
                              out.as_mut_ptr(),
                              kind,
                              flag as u32)
    }
}

impl R2RPlanCreate for f32 {
    unsafe fn r2r_1d(n: usize,
                     in_: &mut RawVec<Self>,
                     out: &mut RawVec<Self>,
                     kind: R2R_KIND,
                     flag: FLAG)
                     -> ffi::fftw_plan {
        ffi::fftwf_plan_r2r_1d(n as i32,
                               in_.as_mut_ptr(),
                               out.as_mut_ptr(),
                               kind,
                               flag as u32)
    }
}

pub trait C2CPlanCreate: Sized {
    unsafe fn c2c_1d(n: usize, in_: &mut RawVec<Self>, out: &mut RawVec<Self>, SIGN, FLAG) -> ffi::fftw_plan;
}

impl C2CPlanCreate for c64 {
    unsafe fn c2c_1d(n: usize, i: &mut RawVec<Self>, o: &mut RawVec<Self>, s: SIGN, f: FLAG) -> ffi::fftw_plan {
        ffi::fftw_plan_dft_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32)
    }
}

impl C2CPlanCreate for c32 {
    unsafe fn c2c_1d(n: usize, i: &mut RawVec<Self>, o: &mut RawVec<Self>, s: SIGN, f: FLAG) -> ffi::fftw_plan {
        ffi::fftwf_plan_dft_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f as u32)
    }
}

pub trait C2RPlanCreate {
    type Real: Sized;
    type Complex: Sized;
    unsafe fn r2c_1d(n: usize, in_: &mut RawVec<Self::Real>, out: &mut RawVec<Self::Complex>, FLAG) -> ffi::fftw_plan;
    unsafe fn c2r_1d(n: usize, in_: &mut RawVec<Self::Complex>, out: &mut RawVec<Self::Real>, FLAG) -> ffi::fftw_plan;
}

impl C2RPlanCreate for (c64, f64) {
    type Real = f64;
    type Complex = c64;
    unsafe fn r2c_1d(n: usize, i: &mut RawVec<Self::Real>, o: &mut RawVec<Self::Complex>, f: FLAG) -> ffi::fftw_plan {
        ffi::fftw_plan_dft_r2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32)
    }
    unsafe fn c2r_1d(n: usize, i: &mut RawVec<Self::Complex>, o: &mut RawVec<Self::Real>, f: FLAG) -> ffi::fftw_plan {
        ffi::fftw_plan_dft_c2r_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32)
    }
}

impl C2RPlanCreate for (c32, f32) {
    type Real = f32;
    type Complex = c32;
    unsafe fn r2c_1d(n: usize, i: &mut RawVec<Self::Real>, o: &mut RawVec<Self::Complex>, f: FLAG) -> ffi::fftw_plan {
        ffi::fftwf_plan_dft_r2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32)
    }
    unsafe fn c2r_1d(n: usize, i: &mut RawVec<Self::Complex>, o: &mut RawVec<Self::Real>, f: FLAG) -> ffi::fftw_plan {
        ffi::fftwf_plan_dft_c2r_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f as u32)
    }
}
