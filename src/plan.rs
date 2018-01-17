use error::*;
use ffi::*;
use types::*;

use std::marker::PhantomData;

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

pub trait C2CPlan {
    type Complex;
    fn new(shape: &[usize], in_: &mut [Self::Complex], out: &mut [Self::Complex], sign: Sign, flag: Flag) -> Result<Self>;
    fn c2c(&mut self, in_: &mut [Self::Complex], out: &mut [Self::Complex]);
}

impl C2CPlan for Plan<c64, c64, Plan64> {
    pub fn new(
        shape: &[usize],
        in_: &mut [c64],
        out: &mut [c64],
        sign: Sign,
        flag: Flag,
    ) -> Result<Self> {
        let plan = excall!{ fftw_plan_dft(
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
}

pub type Plan64 = fftw_plan;
pub type Plan32 = fftwf_plan;

pub trait PlanSpec: Clone + Copy {
    fn validate(self) -> Result<Self>;
    fn destroy(self);
    fn print(self);
}

impl PlanSpec for Plan64 {
    fn validate(self) -> Result<Self> {
        if self.is_null() {
            Err(InvalidPlanError::new().into())
        } else {
            Ok(self)
        }
    }
    fn destroy(self) {
        excall!{ fftw_destroy_plan(self) }
    }
    fn print(self) {
        excall!{ fftw_print_plan(self) }
    }
}

impl PlanSpec for Plan32 {
    fn validate(self) -> Result<Self> {
        if self.is_null() {
            Err(InvalidPlanError::new().into())
        } else {
            Ok(self)
        }
    }
    fn destroy(self) {
        excall!{ fftwf_destroy_plan(self) }
    }
    fn print(self) {
        excall!{ fftwf_print_plan(self) }
    }
}

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
