//! Plan in FFTW
//!
//! See also [Using Plans] in the original document
//! [Using Plans]: http://www.fftw.org/fftw3_doc/Using-Plans.html

use error::*;
use ffi::*;
use types::*;

use std::marker::PhantomData;

pub type C2CPlan64 = Plan<c64, c64, Plan64>;
pub type C2CPlan32 = Plan<c32, c32, Plan32>;
pub type R2CPlan64 = Plan<f64, c64, Plan64>;
pub type R2CPlan32 = Plan<f32, c32, Plan32>;
pub type C2RPlan64 = Plan<c64, f64, Plan64>;
pub type C2RPlan32 = Plan<c32, f32, Plan32>;

/// Typed wrapper of `fftw_plan`
///
/// The plan in FFTW manages the contains all information necessary to compute the transform,
/// including the pointers to the input and output arrays.
/// However, this wrapper *does not modify* this pointer once after the plan is created
/// since it should be *unsafe* in terms of Rust.
/// Instead, this plan executes a transform for different arrays with [new-array execute functions]
/// with related associated functions, e.g. `C2CPlan::c2c`.
///
/// [new-array execute functions]: http://www.fftw.org/fftw3_doc/New_002darray-Execute-Functions.html
pub struct Plan<A, B, Plan: PlanSpec> {
    plan: Plan,
    alignment: Alignment,
    phantom: PhantomData<(A, B)>,
}

impl<A, B, P: PlanSpec> Drop for Plan<A, B, P> {
    fn drop(&mut self) {
        self.plan.destroy();
    }
}

/// Trait for Plan makers
pub trait PlanSpec: Clone + Copy {
    fn validate(self) -> Result<Self>;
    fn destroy(self);
    fn print(self);
}

/// Marker for 64-bit floating point FFT
pub type Plan64 = fftw_plan;
/// Marker for 32-bit floating point FFT
pub type Plan32 = fftwf_plan;

/// Trait for the plan of Complex-to-Complex transformation
pub trait C2CPlan: Sized {
    type Complex;

    /// Create new plan
    fn new(
        shape: &[usize],
        in_: &mut [Self::Complex],
        out: &mut [Self::Complex],
        sign: Sign,
        flag: Flag,
    ) -> Result<Self>;

    /// Execute complex-to-complex transform
    fn c2c(&mut self, in_: &mut [Self::Complex], out: &mut [Self::Complex]) -> Result<()>;
}

/// Trait for the plan of Real-to-Complex transformation
pub trait R2CPlan: Sized {
    type Real;
    type Complex;

    /// Create new plan
    fn new(
        shape: &[usize],
        in_: &mut [Self::Real],
        out: &mut [Self::Complex],
        flag: Flag,
    ) -> Result<Self>;

    /// Execute real-to-complex transform
    fn r2c(&mut self, in_: &mut [Self::Real], out: &mut [Self::Complex]) -> Result<()>;
}

/// Trait for the plan of Complex-to-Real transformation
pub trait C2RPlan: Sized {
    type Real;
    type Complex;

    /// Create new plan
    fn new(
        shape: &[usize],
        in_: &mut [Self::Complex],
        out: &mut [Self::Real],
        flag: Flag,
    ) -> Result<Self>;

    /// Execute complex-to-real transform
    fn c2r(&mut self, in_: &mut [Self::Complex], out: &mut [Self::Real]) -> Result<()>;
}

macro_rules! impl_c2c {
    ($C:ty, $Plan:ty; $plan:ident, $exec:ident) => {
        impl C2CPlan for Plan<$C, $C, $Plan> {
            type Complex = $C;
            fn new(
                shape: &[usize],
                in_: &mut [Self::Complex],
                out: &mut [Self::Complex],
                sign: Sign,
                flag: Flag,
            ) -> Result<Self> {
                let plan = excall!{ $plan(
                    shape.len() as i32,
                    shape.to_cint().as_mut_ptr() as *mut _,
                    in_.as_mut_ptr(),
                    out.as_mut_ptr(),
                    sign as i32, flag.into())
                }.validate()?;
                Ok(Self {
                    plan,
                    alignment: Alignment::new(in_, out),
                    phantom: PhantomData,
                })
            }
            fn c2c(&mut self, in_: &mut [Self::Complex], out: &mut [Self::Complex]) -> Result<()> {
                self.alignment.check(in_, out)?;
                unsafe { $exec(self.plan, in_.as_mut_ptr(), out.as_mut_ptr()) };
                Ok(())
            }
        }
    };
} // impl_c2c!

impl_c2c!(c64, Plan64; fftw_plan_dft, fftw_execute_dft);
impl_c2c!(c32, Plan32; fftwf_plan_dft, fftwf_execute_dft);

macro_rules! impl_r2c {
    ($R:ty, $C:ty, $Plan:ty; $plan:ident, $exec:ident) => {
        impl R2CPlan for Plan<$R, $C, $Plan> {
            type Real = $R;
            type Complex = $C;
            fn new(
                shape: &[usize],
                in_: &mut [Self::Real],
                out: &mut [Self::Complex],
                flag: Flag,
            ) -> Result<Self> {
                let plan = excall!{ $plan(
                    shape.len() as i32,
                    shape.to_cint().as_mut_ptr() as *mut _,
                    in_.as_mut_ptr(),
                    out.as_mut_ptr(),
                    flag.into())
                }.validate()?;
                Ok(Self {
                    plan,
                    alignment: Alignment::new(in_, out),
                    phantom: PhantomData,
                })
            }
            fn r2c(&mut self, in_: &mut [Self::Real], out: &mut [Self::Complex]) -> Result<()> {
                self.alignment.check(in_, out)?;
                unsafe { $exec(self.plan, in_.as_mut_ptr(), out.as_mut_ptr()) };
                Ok(())
            }
        }
    };
} // impl_r2c!

impl_r2c!(f64, c64, Plan64; fftw_plan_dft_r2c, fftw_execute_dft_r2c);
impl_r2c!(f32, c32, Plan32; fftwf_plan_dft_r2c, fftwf_execute_dft_r2c);

macro_rules! impl_c2r {
    ($R:ty, $C:ty, $Plan:ty; $plan:ident, $exec:ident) => {
        impl C2RPlan for Plan<$C, $R, $Plan> {
            type Real = $R;
            type Complex = $C;
            fn new(
                shape: &[usize],
                in_: &mut [Self::Complex],
                out: &mut [Self::Real],
                flag: Flag,
            ) -> Result<Self> {
                let plan = excall!{ $plan(
                    shape.len() as i32,
                    shape.to_cint().as_mut_ptr() as *mut _,
                    in_.as_mut_ptr(),
                    out.as_mut_ptr(),
                    flag.into())
                }.validate()?;
                Ok(Self {
                    plan,
                    alignment: Alignment::new(in_, out),
                    phantom: PhantomData,
                })
            }
            fn c2r(&mut self, in_: &mut [Self::Complex], out: &mut [Self::Real]) -> Result<()> {
                self.alignment.check(in_, out)?;
                unsafe { $exec(self.plan, in_.as_mut_ptr(), out.as_mut_ptr()) };
                Ok(())
            }
        }
    };
} // impl_c2r!

impl_c2r!(f64, c64, Plan64; fftw_plan_dft_c2r, fftw_execute_dft_c2r);
impl_c2r!(f32, c32, Plan32; fftwf_plan_dft_c2r, fftwf_execute_dft_c2r);

macro_rules! impl_plan_spec {
    ($Plan:ty; $destroy_plan:ident, $print_plan:ident) => {
        impl PlanSpec for $Plan {
            fn validate(self) -> Result<Self> {
                if self.is_null() {
                    Err(InvalidPlanError::new().into())
                } else {
                    Ok(self)
                }
            }
            fn destroy(self) {
                excall!{ $destroy_plan(self) }
            }
            fn print(self) {
                excall!{ $print_plan(self) }
            }
        }
    };
} // impl_plan_spec!

impl_plan_spec!(Plan64; fftw_destroy_plan, fftw_print_plan);
impl_plan_spec!(Plan32; fftwf_destroy_plan, fftwf_print_plan);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Alignment {
    in_: i32,
    out: i32,
    n_in_: usize,
    n_out: usize,
}

fn alignment_of<T>(a: &[T]) -> i32 {
    unsafe { fftw_alignment_of(a.as_ptr() as *mut _) }
}

impl Alignment {
    fn new<A, B>(in_: &[A], out: &[B]) -> Self {
        Self {
            in_: alignment_of(in_),
            out: alignment_of(out),
            n_in_: in_.len(),
            n_out: out.len(),
        }
    }

    fn check<A, B>(&self, in_: &[A], out: &[B]) -> Result<()> {
        let args = Self::new(in_, out);
        if *self != args {
            Err(InputMismatchError {
                origin: *self,
                args,
            }.into())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct InputMismatchError {
    origin: Alignment,
    args: Alignment,
}

trait ToCInt {
    fn to_cint(&self) -> Vec<i32>;
}

impl ToCInt for [usize] {
    fn to_cint(&self) -> Vec<i32> {
        self.iter().map(|&x| x as i32).collect()
    }
}
