use super::*;
use error::*;
use ffi::*;

#[derive(Debug, Clone)]
struct Alignment {
    in_: i32,
    out: i32,
}

impl Alignment {
    fn new<A: FFTW, B: FFTW>(in_: &[A], out: &[B]) -> Self {
        Self {
            in_: A::alignment_of(in_),
            out: B::alignment_of(out),
        }
    }
    fn check<A: FFTW, B: FFTW>(&self, in_: &[A], out: &[B]) -> Result<()> {
        if self.in_ != A::alignment_of(in_) || self.out != B::alignment_of(out) {
            Err(AlignmentMismatchError::new().into())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
struct Shape(Vec<i32>);

impl Shape {
    fn new(s: &[i32]) -> Self {
        Shape(s.to_vec())
    }
    fn check<A: FFTW, B: FFTW>(&self, in_: &[A], out: &[B]) -> Result<()> {
        let n = self.0.len();
        if in_.len() != n || out.len() != n {
            Err(SizeMismatchError::new().into())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct C2CPlan<C: FFTW> {
    plan: C::Plan,
    shape: Shape,
    sign: Sign,
    flag: FLAG,
    alignment: Alignment,
}

impl<C: FFTW> Drop for C2CPlan<C> {
    fn drop(&mut self) {
        C::destroy_plan(self.plan);
    }
}

impl<C: FFTW<Complex = C>> C2CPlan<C> {
    pub fn new(shape: &[i32], in_: &[C], out: &[C], sign: Sign, flag: FLAG) -> Result<Self> {
        Ok(Self {
            plan: C::plan_c2c(&shape, in_, out, sign, flag)?,
            shape: Shape::new(shape),
            sign,
            flag,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn c2c(&mut self, in_: &[C], out: &mut [C]) -> Result<()> {
        self.alignment.check(in_, out)?;
        self.shape.check(in_, out)?;
        C::exec_c2c(self.plan, in_, out);
        Ok(())
    }
}

#[derive(Debug)]
pub struct C2RPlan<C, R>
where
    C: FFTW<Real = R, Complex = C>,
    R: FFTW<Real = R, Complex = C, Plan = C::Plan>,
{
    plan: C::Plan,
    shape: Shape,
    flag: FLAG,
    alignment: Alignment,
}

impl<C, R> Drop for C2RPlan<C, R>
where
    C: FFTW<Real = R, Complex = C>,
    R: FFTW<Real = R, Complex = C, Plan = C::Plan>,
{
    fn drop(&mut self) {
        R::destroy_plan(self.plan);
    }
}

impl<C, R> C2RPlan<C, R>
where
    C: FFTW<Real = R, Complex = C>,
    R: FFTW<Real = R, Complex = C, Plan = C::Plan>,
{
    pub fn new(shape: &[i32], in_: &[C], out: &[R], flag: FLAG) -> Result<Self> {
        Ok(Self {
            plan: C::plan_c2r(&shape, in_, out, flag)?,
            shape: Shape::new(shape),
            flag,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn c2r(&mut self, in_: &[C], out: &mut [R]) -> Result<()> {
        self.alignment.check(in_, out)?;
        self.shape.check(in_, out)?;
        C::exec_c2r(self.plan, in_, out);
        Ok(())
    }
}

#[derive(Debug)]
pub struct R2CPlan<R, C>
where
    C: FFTW<Real = R, Complex = C>,
    R: FFTW<Real = R, Complex = C, Plan = C::Plan>,
{
    plan: C::Plan,
    shape: Shape,
    flag: FLAG,
    alignment: Alignment,
}

impl<R, C> Drop for R2CPlan<R, C>
where
    C: FFTW<Real = R, Complex = C>,
    R: FFTW<Real = R, Complex = C, Plan = C::Plan>,
{
    fn drop(&mut self) {
        R::destroy_plan(self.plan);
    }
}

impl<R, C> R2CPlan<R, C>
where
    C: FFTW<Real = R, Complex = C>,
    R: FFTW<Real = R, Complex = C, Plan = C::Plan>,
{
    pub fn new(shape: &[i32], in_: &[R], out: &[C], flag: FLAG) -> Result<Self> {
        Ok(Self {
            plan: C::plan_r2c(&shape, in_, out, flag)?,
            shape: Shape::new(shape),
            flag,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn r2c(&mut self, in_: &[R], out: &mut [C]) -> Result<()> {
        self.alignment.check(in_, out)?;
        self.shape.check(in_, out)?;
        C::exec_r2c(self.plan, in_, out);
        Ok(())
    }
}

pub type R2RKind = ffi::fftw_r2r_kind;

#[derive(Debug)]
pub struct R2RPlan<R: FFTW> {
    plan: R::Plan,
    shape: Shape,
    kind: Vec<R2RKind>,
    flag: FLAG,
    alignment: Alignment,
}

impl<R: FFTW> Drop for R2RPlan<R> {
    fn drop(&mut self) {
        R::destroy_plan(self.plan);
    }
}

impl<R: FFTW<Real = R>> R2RPlan<R> {
    pub fn new(shape: &[i32], in_: &[R], out: &[R], kinds: &[R2RKind], flag: FLAG) -> Result<Self> {
        Ok(Self {
            plan: R::plan_r2r(&shape, in_, out, kinds, flag)?,
            shape: Shape::new(shape),
            kind: kinds.to_vec(),
            flag,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn r2c(&mut self, in_: &[R], out: &mut [R]) -> Result<()> {
        self.alignment.check(in_, out)?;
        self.shape.check(in_, out)?;
        R::exec_r2r(self.plan, in_, out);
        Ok(())
    }
}

pub trait Plan: Sized {
    fn check_null(self) -> Result<Self>;
}

impl Plan for fftw_plan {
    fn check_null(self) -> Result<Self> {
        if self.is_null() {
            Err(InvalidPlanError::new().into())
        } else {
            Ok(self)
        }
    }
}

impl Plan for fftwf_plan {
    fn check_null(self) -> Result<Self> {
        if self.is_null() {
            Err(InvalidPlanError::new().into())
        } else {
            Ok(self)
        }
    }
}

/// Switch `fftw_*` and `fftwf_*`
pub trait FFTW {
    type Plan: Plan + Copy;
    type Real;
    type Complex;
    fn destroy_plan(Self::Plan);
    fn print_plan(Self::Plan);
    fn plan_c2c(shape: &[i32], in_: &[Self::Complex], out: &[Self::Complex], Sign, FLAG) -> Result<Self::Plan>;
    fn plan_c2r(shape: &[i32], in_: &[Self::Complex], out: &[Self::Real], FLAG) -> Result<Self::Plan>;
    fn plan_r2c(shape: &[i32], in_: &[Self::Real], out: &[Self::Complex], FLAG) -> Result<Self::Plan>;
    fn plan_r2r(shape: &[i32], in_: &[Self::Real], out: &[Self::Real], &[R2RKind], FLAG) -> Result<Self::Plan>;
    fn exec_c2c(p: Self::Plan, in_: &[Self::Complex], &mut [Self::Complex]);
    fn exec_c2r(p: Self::Plan, in_: &[Self::Complex], &mut [Self::Real]);
    fn exec_r2c(p: Self::Plan, in_: &[Self::Real], &mut [Self::Complex]);
    fn exec_r2r(p: Self::Plan, in_: &[Self::Real], &mut [Self::Real]);
    fn alignment_of<T>(&[T]) -> i32;
}

macro_rules! excall {
    ($call:expr) => {
        let _lock = FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe { $call }
    }
}

impl FFTW for c64 {
    type Plan = fftw_plan;
    type Real = f64;
    type Complex = c64;
    fn destroy_plan(p: Self::Plan) {
        excall!{ fftw_destroy_plan(p) };
    }
    fn print_plan(p: Self::Plan) {
        excall!{ fftw_print_plan(p) };
    }
    fn plan_c2c(
        shape: &[i32],
        in_: &[Self::Complex],
        out: &[Self::Complex],
        sign: Sign,
        flag: FLAG,
    ) -> Result<Self::Plan> {
        excall!{ fftw_plan_dft(shape.len() as i32, shape.as_ptr(), in_.as_ptr() as *mut _, out.as_ptr() as *mut _, sign, flag).check_null() }
    }
    fn plan_c2r(shape: &[i32], in_: &[Self::Complex], out: &[Self::Real], flag: FLAG) -> Result<Self::Plan> {
        excall!{ fftw_plan_dft_c2r(shape.len() as i32, shape.as_ptr(), in_.as_ptr() as *mut _, out.as_ptr() as *mut _, flag).check_null() }
    }
    fn plan_r2c(shape: &[i32], in_: &[Self::Real], out: &[Self::Complex], flag: FLAG) -> Result<Self::Plan> {
        excall!{ fftw_plan_dft_r2c(shape.len() as i32, shape.as_ptr(), in_.as_ptr() as *mut _, out.as_ptr() as *mut _, flag).check_null() }
    }
    fn plan_r2r(
        shape: &[i32],
        in_: &[Self::Real],
        out: &[Self::Real],
        kinds: &[R2RKind],
        flag: FLAG,
    ) -> Result<Self::Plan> {
        excall!{ fftw_plan_r2r(shape.len() as i32, shape.as_ptr(), in_.as_ptr() as *mut _, out.as_ptr() as *mut _, kinds.as_ptr() as *mut _, flag).check_null() }
    }
    fn exec_c2c(p: Self::Plan, in_: &[Self::Complex], out: &mut [Self::Complex]) {
        unsafe { fftw_execute_dft(p, in_.as_ptr() as *mut _, out.as_mut_ptr()) };
    }
    fn exec_c2r(p: Self::Plan, in_: &[Self::Complex], out: &mut [Self::Real]) {
        unsafe { fftw_execute_dft_c2r(p, in_.as_ptr() as *mut _, out.as_mut_ptr()) };
    }
    fn exec_r2c(p: Self::Plan, in_: &[Self::Real], out: &mut [Self::Complex]) {
        unsafe { fftw_execute_dft_r2c(p, in_.as_ptr() as *mut _, out.as_mut_ptr()) };
    }
    fn exec_r2r(p: Self::Plan, in_: &[Self::Real], out: &mut [Self::Real]) {
        unsafe { fftw_execute_r2r(p, in_.as_ptr() as *mut _, out.as_mut_ptr()) };
    }
    fn alignment_of<T>(s: &[T]) -> i32 {
        unsafe { fftw_alignment_of(s.as_ptr() as *mut _) }
    }
}
