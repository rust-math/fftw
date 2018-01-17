use error::*;
use ffi::*;
use types::*;

use std::marker::PhantomData;

pub type Plan64 = fftw_plan;
pub type Plan32 = fftwf_plan;

pub trait PlanSpec: Clone + Copy {
    fn validate(self) -> Result<Self>;
    fn destroy(self);
    fn print(self);
}

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
    fn c2c(
        &mut self,
        in_: &mut [Self::Complex],
        out: &mut [Self::Complex]
    ) -> Result<()>;
}

macro_rules! impl_c2c { ($C:ty, $Plan:ty; $plan:ident, $exec:ident) => {
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
}} // impl_c2c!

impl_c2c!(c64, Plan64; fftw_plan_dft, fftw_execute_dft);
impl_c2c!(c32, Plan32; fftwf_plan_dft, fftwf_execute_dft);

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
}} // impl_plan_spec!

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
