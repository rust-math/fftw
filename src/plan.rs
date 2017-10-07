use super::{FLAG, R2R_KIND, SIGN, c32, c64};
use super::aligned_vec::AlignedVec;
use super::util::FFTW_MUTEX;
use ffi;

use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::null;

pub struct Plan<A, B> {
    plan: RawPlan,
    phantom: PhantomData<(A, B)>,
}

impl<A, B> Plan<A, B> {
    /// this function modifys the array referred in plan creation
    pub unsafe fn execute(&self) {
        self.plan.execute()
    }
}

impl<T: R2RPlanCreate> Plan<T, T> {
    pub fn r2r_1d(
        n: usize,
        mut in_: &mut AlignedVec<T>,
        mut out: &mut AlignedVec<T>,
        kind: R2R_KIND,
        flag: FLAG,
    ) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { T::r2r_1d(n, &mut in_, &mut out, kind, flag) },
            phantom: PhantomData,
        }
    }
    pub fn r2r_2d(
        n0: usize,
        n1: usize,
        mut in_: &mut AlignedVec<T>,
        mut out: &mut AlignedVec<T>,
        k0: R2R_KIND,
        k1: R2R_KIND,
        flag: FLAG,
    ) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { T::r2r_2d(n0, n1, &mut in_, &mut out, k0, k1, flag) },
            phantom: PhantomData,
        }
    }
    pub fn r2r_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        mut in_: &mut AlignedVec<T>,
        mut out: &mut AlignedVec<T>,
        k0: R2R_KIND,
        k1: R2R_KIND,
        k2: R2R_KIND,
        flag: FLAG,
    ) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { T::r2r_3d(n0, n1, n2, &mut in_, &mut out, k0, k1, k2, flag) },
            phantom: PhantomData,
        }
    }
}

impl<T: C2CPlanCreate> Plan<T, T> {
    pub fn c2c_1d(n: usize, mut in_: &mut AlignedVec<T>, mut out: &mut AlignedVec<T>, sign: SIGN, flag: FLAG) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { T::c2c_1d(n, &mut in_, &mut out, sign, flag) },
            phantom: PhantomData,
        }
    }
    pub fn c2c_2d(
        n0: usize,
        n1: usize,
        mut in_: &mut AlignedVec<T>,
        mut out: &mut AlignedVec<T>,
        sign: SIGN,
        flag: FLAG,
    ) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { T::c2c_2d(n0, n1, &mut in_, &mut out, sign, flag) },
            phantom: PhantomData,
        }
    }
    pub fn c2c_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        mut in_: &mut AlignedVec<T>,
        mut out: &mut AlignedVec<T>,
        sign: SIGN,
        flag: FLAG,
    ) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { T::c2c_3d(n0, n1, n2, &mut in_, &mut out, sign, flag) },
            phantom: PhantomData,
        }
    }
}

impl<C, R> Plan<C, R>
where
    (C, R): C2RPlanCreate<Real = R, Complex = C>,
{
    pub fn c2r_1d(n: usize, in_: &mut AlignedVec<C>, out: &mut AlignedVec<R>, flag: FLAG) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { <(C, R)>::c2r_1d(n, in_, out, flag) },
            phantom: PhantomData,
        }
    }
    pub fn c2r_2d(n0: usize, n1: usize, in_: &mut AlignedVec<C>, out: &mut AlignedVec<R>, flag: FLAG) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { <(C, R)>::c2r_2d(n0, n1, in_, out, flag) },
            phantom: PhantomData,
        }
    }
    pub fn c2r_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        in_: &mut AlignedVec<C>,
        out: &mut AlignedVec<R>,
        flag: FLAG,
    ) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { <(C, R)>::c2r_3d(n0, n1, n2, in_, out, flag) },
            phantom: PhantomData,
        }
    }
}

impl<R, C> Plan<R, C>
where
    (C, R): C2RPlanCreate<Real = R, Complex = C>,
{
    pub fn r2c_1d(n: usize, in_: &mut AlignedVec<R>, out: &mut AlignedVec<C>, flag: FLAG) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { <(C, R)>::r2c_1d(n, in_, out, flag) },
            phantom: PhantomData,
        }
    }
    pub fn r2c_2d(n0: usize, n1: usize, in_: &mut AlignedVec<R>, out: &mut AlignedVec<C>, flag: FLAG) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { <(C, R)>::r2c_2d(n0, n1, in_, out, flag) },
            phantom: PhantomData,
        }
    }
    pub fn r2c_3d(
        n0: usize,
        n1: usize,
        n2: usize,
        in_: &mut AlignedVec<R>,
        out: &mut AlignedVec<C>,
        flag: FLAG,
    ) -> Self {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        Plan {
            plan: unsafe { <(C, R)>::r2c_3d(n0, n1, n2, in_, out, flag) },
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
        self.null_check();
        match *self {
            RawPlan::_64(p) => ffi::fftw_execute(p),
            RawPlan::_32(p) => ffi::fftwf_execute(p),
        }
    }

    fn null_check(&self) {
        let p = match *self {
            RawPlan::_64(p) => p as *const c_void,
            RawPlan::_32(p) => p as *const c_void,
        };
        if p == null() {
            panic!(
                "Plan is NULL. If you use MKL binding, check here: https://software.intel.com/en-us/mkl-developer-reference-c-using-fftw3-wrappers"
            );
        }
    }
}

impl Drop for RawPlan {
    fn drop(&mut self) {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe {
            match *self {
                RawPlan::_64(p) => ffi::fftw_destroy_plan(p),
                RawPlan::_32(p) => ffi::fftwf_destroy_plan(p),
            }
        }
    }
}

pub trait R2RPlanCreate: Sized {
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
pub trait C2CPlanCreate: Sized {
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

pub trait C2RPlanCreate {
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

impl R2RPlanCreate for $float {
    unsafe fn r2r_1d(n: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, kind: R2R_KIND, flag: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2r_1d(n as i32, in_.as_mut_ptr(), out.as_mut_ptr(), kind, flag ))
    }
    unsafe fn r2r_2d(n0: usize, n1: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, k0: R2R_KIND, k1: R2R_KIND, flag: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2r_2d(n0 as i32, n1 as i32, in_.as_mut_ptr(), out.as_mut_ptr(), k0, k1, flag ))
    }
    unsafe fn r2r_3d(n0: usize, n1: usize, n2: usize, in_: &mut AlignedVec<Self>, out: &mut AlignedVec<Self>, k0: R2R_KIND, k1: R2R_KIND, k2: R2R_KIND, flag: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2r_3d(n0 as i32, n1 as i32, n2 as i32, in_.as_mut_ptr(), out.as_mut_ptr(), k0, k1, k2, flag ))
    }
}

impl C2CPlanCreate for $complex {
    unsafe fn c2c_1d(n: usize, i: &mut AlignedVec<Self>, o: &mut AlignedVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f ))
    }
    unsafe fn c2c_2d(n0: usize, n1: usize, i: &mut AlignedVec<Self>, o: &mut AlignedVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2c_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f ))
    }
    unsafe fn c2c_3d(n0: usize, n1: usize, n2: usize, i: &mut AlignedVec<Self>, o: &mut AlignedVec<Self>, s: SIGN, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2c_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), s as i32, f ))
    }
}

impl C2RPlanCreate for ($complex, $float) {
    type Real = $float;
    type Complex = $complex;
    unsafe fn r2c_1d(n: usize, i: &mut AlignedVec<Self::Real>, o: &mut AlignedVec<Self::Complex>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2c_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f ))
    }
    unsafe fn c2r_1d(n: usize, i: &mut AlignedVec<Self::Complex>, o: &mut AlignedVec<Self::Real>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2r_1d(n as i32, i.as_mut_ptr(), o.as_mut_ptr(), f ))
    }
    unsafe fn r2c_2d(n0: usize, n1: usize, i: &mut AlignedVec<Self::Real>, o: &mut AlignedVec<Self::Complex>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2c_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f ))
    }
    unsafe fn c2r_2d(n0: usize, n1: usize, i: &mut AlignedVec<Self::Complex>, o: &mut AlignedVec<Self::Real>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2r_2d(n0 as i32, n1 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f ))
    }
    unsafe fn r2c_3d(n0: usize, n1: usize, n2: usize, i: &mut AlignedVec<Self::Real>, o: &mut AlignedVec<Self::Complex>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$r2c_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f ))
    }
    unsafe fn c2r_3d(n0: usize, n1: usize, n2: usize, i: &mut AlignedVec<Self::Complex>, o: &mut AlignedVec<Self::Real>, f: FLAG) -> RawPlan {
        RawPlan::$bit(ffi::$c2r_3d(n0 as i32, n1 as i32, n2 as i32, i.as_mut_ptr(), o.as_mut_ptr(), f ))
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
