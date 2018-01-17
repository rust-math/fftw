use error::*;
use ffi::*;
use types::*;

#[derive(Debug)]
pub struct C2CPlan<C: FFTW> {
    plan: C::Plan,
    alignment: Alignment,
}

impl<C: FFTW> Drop for C2CPlan<C> {
    fn drop(&mut self) {
        C::destroy_plan(self.plan);
    }
}

impl<C: FFTW<Complex = C>> C2CPlan<C> {
    pub fn new(
        shape: &[usize],
        in_: &mut [C],
        out: &mut [C],
        sign: Sign,
        flag: Flag,
    ) -> Result<Self> {
        Ok(Self {
            plan: C::plan_c2c(&shape.to_cint(), in_, out, sign, flag)?,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn c2c(&mut self, in_: &mut [C], out: &mut [C]) -> Result<()> {
        self.alignment.check(in_, out)?;
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
    pub fn new(shape: &[usize], in_: &mut [C], out: &mut [R], flag: Flag) -> Result<Self> {
        Ok(Self {
            plan: C::plan_c2r(&shape.to_cint(), in_, out, flag)?,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn c2r(&mut self, in_: &mut [C], out: &mut [R]) -> Result<()> {
        self.alignment.check(in_, out)?;
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
    pub fn new(shape: &[usize], in_: &mut [R], out: &mut [C], flag: Flag) -> Result<Self> {
        Ok(Self {
            plan: C::plan_r2c(&shape.to_cint(), in_, out, flag)?,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn r2c(&mut self, in_: &mut [R], out: &mut [C]) -> Result<()> {
        self.alignment.check(in_, out)?;
        C::exec_r2c(self.plan, in_, out);
        Ok(())
    }
}

#[derive(Debug)]
pub struct R2RPlan<R: FFTW> {
    plan: R::Plan,
    alignment: Alignment,
}

impl<R: FFTW> Drop for R2RPlan<R> {
    fn drop(&mut self) {
        R::destroy_plan(self.plan);
    }
}

impl<R: FFTW<Real = R>> R2RPlan<R> {
    pub fn new(
        shape: &[usize],
        in_: &mut [R],
        out: &mut [R],
        kinds: &[R2RKind],
        flag: Flag,
    ) -> Result<Self> {
        Ok(Self {
            plan: R::plan_r2r(&shape.to_cint(), in_, out, kinds, flag)?,
            alignment: Alignment::new(in_, out),
        })
    }
    pub fn r2c(&mut self, in_: &mut [R], out: &mut [R]) -> Result<()> {
        self.alignment.check(in_, out)?;
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
    fn plan_c2c(
        shape: &[i32],
        in_: &mut [Self::Complex],
        out: &mut [Self::Complex],
        Sign,
        Flag,
    ) -> Result<Self::Plan>;
    fn plan_c2r(
        shape: &[i32],
        in_: &mut [Self::Complex],
        out: &mut [Self::Real],
        Flag,
    ) -> Result<Self::Plan>;
    fn plan_r2c(
        shape: &[i32],
        in_: &mut [Self::Real],
        out: &mut [Self::Complex],
        Flag,
    ) -> Result<Self::Plan>;
    fn plan_r2r(
        shape: &[i32],
        in_: &mut [Self::Real],
        out: &mut [Self::Real],
        &[R2RKind],
        Flag,
    ) -> Result<Self::Plan>;
    fn exec_c2c(p: Self::Plan, in_: &mut [Self::Complex], out: &mut [Self::Complex]);
    fn exec_c2r(p: Self::Plan, in_: &mut [Self::Complex], out: &mut [Self::Real]);
    fn exec_r2c(p: Self::Plan, in_: &mut [Self::Real], out: &mut [Self::Complex]);
    fn exec_r2r(p: Self::Plan, in_: &mut [Self::Real], out: &mut [Self::Real]);
    fn alignment_of<T>(&[T]) -> i32;
}

macro_rules! impl_fftw { ($scalar:ty) => {
impl FFTW for $scalar {
    type Real = f64;
    type Complex = c64;
    type Plan = fftw_plan;
    fn destroy_plan(p: Self::Plan) {
        excall!{ fftw_destroy_plan(p) };
    }
    fn print_plan(p: Self::Plan) {
        excall!{ fftw_print_plan(p) };
    }
    fn plan_c2c(shape: &[i32], in_: &mut [Self::Complex], out: &mut [Self::Complex], sign: Sign, flag: Flag) -> Result<Self::Plan> {
        excall!{ fftw_plan_dft(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), sign as i32, flag.into()).check_null() }
    }
    fn plan_c2r(shape: &[i32], in_: &mut [Self::Complex], out: &mut [Self::Real], flag: Flag) -> Result<Self::Plan> {
        excall!{ fftw_plan_dft_c2r(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), flag.into()).check_null() }
    }
    fn plan_r2c(shape: &[i32], in_: &mut [Self::Real], out: &mut [Self::Complex], flag: Flag) -> Result<Self::Plan> {
        excall!{ fftw_plan_dft_r2c(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), flag.into()).check_null() }
    }
    fn plan_r2r(shape: &[i32], in_: &mut [Self::Real], out: &mut [Self::Real], kinds: &[R2RKind], flag: Flag) -> Result<Self::Plan> {
        excall!{ fftw_plan_r2r(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), kinds.as_ptr(), flag.into()).check_null() }
    }
    fn exec_c2c(p: Self::Plan, in_: &mut [Self::Complex], out: &mut [Self::Complex]) {
        unsafe { fftw_execute_dft(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn exec_c2r(p: Self::Plan, in_: &mut [Self::Complex], out: &mut [Self::Real]) {
        unsafe { fftw_execute_dft_c2r(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn exec_r2c(p: Self::Plan, in_: &mut [Self::Real], out: &mut [Self::Complex]) {
        unsafe { fftw_execute_dft_r2c(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn exec_r2r(p: Self::Plan, in_: &mut [Self::Real], out: &mut [Self::Real]) {
        unsafe { fftw_execute_r2r(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn alignment_of<T>(s: &[T]) -> i32 {
        unsafe { fftw_alignment_of(s.as_ptr() as *mut _) }
    }
}
}} // impl_fftw

impl_fftw!(f64);
impl_fftw!(c64);

macro_rules! impl_fftwf { ($scalar:ty) => {
impl FFTW for $scalar {
    type Real = f32;
    type Complex = c32;
    type Plan = fftwf_plan;
    fn destroy_plan(p: Self::Plan) {
        excall!{ fftwf_destroy_plan(p) };
    }
    fn print_plan(p: Self::Plan) {
        excall!{ fftwf_print_plan(p) };
    }
    fn plan_c2c(shape: &[i32], in_: &mut [Self::Complex], out: &mut [Self::Complex], sign: Sign, flag: Flag) -> Result<Self::Plan> {
        excall!{ fftwf_plan_dft(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), sign as i32, flag.into()).check_null() }
    }
    fn plan_c2r(shape: &[i32], in_: &mut [Self::Complex], out: &mut [Self::Real], flag: Flag) -> Result<Self::Plan> {
        excall!{ fftwf_plan_dft_c2r(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), flag.into()).check_null() }
    }
    fn plan_r2c(shape: &[i32], in_: &mut [Self::Real], out: &mut [Self::Complex], flag: Flag) -> Result<Self::Plan> {
        excall!{ fftwf_plan_dft_r2c(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), flag.into()).check_null() }
    }
    fn plan_r2r(shape: &[i32], in_: &mut [Self::Real], out: &mut [Self::Real], kinds: &[R2RKind], flag: Flag) -> Result<Self::Plan> {
        excall!{ fftwf_plan_r2r(shape.len() as i32, shape.as_ptr(), in_.as_mut_ptr(), out.as_mut_ptr(), kinds.as_ptr(), flag.into()).check_null() }
    }
    fn exec_c2c(p: Self::Plan, in_: &mut [Self::Complex], out: &mut [Self::Complex]) {
        unsafe { fftwf_execute_dft(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn exec_c2r(p: Self::Plan, in_: &mut [Self::Complex], out: &mut [Self::Real]) {
        unsafe { fftwf_execute_dft_c2r(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn exec_r2c(p: Self::Plan, in_: &mut [Self::Real], out: &mut [Self::Complex]) {
        unsafe { fftwf_execute_dft_r2c(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn exec_r2r(p: Self::Plan, in_: &mut [Self::Real], out: &mut [Self::Real]) {
        unsafe { fftwf_execute_r2r(p, in_.as_mut_ptr(), out.as_mut_ptr()) };
    }
    fn alignment_of<T>(s: &[T]) -> i32 {
        unsafe { fftwf_alignment_of(s.as_ptr() as *mut _) }
    }
}
}} // impl_fftwf

impl_fftwf!(f32);
impl_fftwf!(c32);

#[derive(Debug)]
pub struct NAEInputMismatchError {
    origin: Alignment,
    args: Alignment,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Alignment {
    in_: i32,
    out: i32,
    n_in_: usize,
    n_out: usize,
}

impl Alignment {
    fn new<A: FFTW, B: FFTW>(in_: &[A], out: &[B]) -> Self {
        Self {
            in_: A::alignment_of(in_),
            out: B::alignment_of(out),
            n_in_: in_.len(),
            n_out: out.len(),
        }
    }
    fn check<A: FFTW, B: FFTW>(&self, in_: &[A], out: &[B]) -> Result<()> {
        let args = Self::new(in_, out);
        if *self != args {
            Err(NAEInputMismatchError {
                origin: *self,
                args,
            }.into())
        } else {
            Ok(())
        }
    }
}

trait ToCInt {
    fn to_cint(&self) -> Vec<i32>;
}

impl ToCInt for [usize] {
    fn to_cint(&self) -> Vec<i32> {
        self.iter().map(|&x| x as i32).collect()
    }
}
