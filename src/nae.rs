use super::*;
use ffi::*;

#[derive(Debug)]
pub struct C2CPlan<C: FFTW> {
    plan: C::Plan,
    shape: Vec<i32>,
    flag: FLAG,
    a_in_: i32,
    a_out: i32,
}

impl<C: FFTW<Complex = C>> C2CPlan<C> {
    pub fn new(shape: &[i32], in_: &[C], out: &[C], sign: Sign, flag: FLAG) -> Self {
        let shape = shape.to_vec();
        let plan = C::plan_c2c(&shape, in_, out, sign, flag);
        let a_in_ = C::alignment_of(in_);
        let a_out = C::alignment_of(out);
        Self { plan, shape, flag, a_in_, a_out }
    }
    pub fn c2c(&mut self, in_: &[C], out: &mut [C]) {
        if self.a_in_ != C::alignment_of(in_) || self.a_out != C::alignment_of(out) {
            panic!("Alignment is mismatched");
        }
        C::execute_c2c(self.plan, in_, out);
    }
}

/// Switch `fftw_*` and `fftwf_*`
pub trait FFTW {
    type Plan: Copy;
    type Real;
    type Complex;
    fn destroy_plan(Self::Plan);
    fn print_plan(Self::Plan);
    fn plan_c2c(shape: &[i32], in_: &[Self::Complex], out: &[Self::Complex], sign: Sign, flags: FLAG) -> Self::Plan;
    fn execute_c2c(p: Self::Plan, in_: &[Self::Complex], &mut [Self::Complex]);
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
    fn plan_c2c(shape: &[i32], in_: &[Self::Complex], out: &[Self::Complex], sign: Sign, flag: FLAG) -> Self::Plan {
        excall!{ fftw_plan_dft(shape.len() as i32, shape.as_ptr(), in_.as_ptr() as *mut _, out.as_ptr() as *mut _, sign, flag) }
    }
    fn execute_c2c(p: Self::Plan, in_: &[Self::Complex], out: &mut [Self::Complex]) {
        unsafe { fftw_execute_dft(p, in_.as_ptr() as *mut _, out.as_mut_ptr()) };
    }
    fn alignment_of<T>(s: &[T]) -> i32 {
        unsafe { fftw_alignment_of(s.as_ptr() as *mut _) }
    }
}
