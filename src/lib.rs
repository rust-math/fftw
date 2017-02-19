#![allow(non_camel_case_types)]

extern crate fftw3_sys as ffi;

pub use ffi::fftw_r2r_kind as R2R_KIND;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SIGN {
    FFTW_FORWARD = -1,
    FFTW_BACKWARD = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FLAG {
    /* documented flags */
    FFTW_MEASURE = 0,
    FFTW_DESTROY_INPUT = 1 << 0,
    FFTW_UNALIGNED = 1 << 1,
    FFTW_CONSERVE_MEMORY = 1 << 2,
    FFTW_EXHAUSTIVE = 1 << 3, /* NO_EXHAUSTIVE is default */
    FFTW_PRESERVE_INPUT = 1 << 4, /* cancels FFTW_DESTROY_INPUT */
    FFTW_PATIENT = 1 << 5, /* IMPATIENT is default */
    FFTW_ESTIMATE = 1 << 6,
    FFTW_WISDOM_ONLY = 1 << 21,

    /* undocumented beyond-guru flags */
    FFTW_ESTIMATE_PATIENT = 1 << 7,
    FFTW_BELIEVE_PCOST = 1 << 8,
    FFTW_NO_DFT_R2HC = 1 << 9,
    FFTW_NO_NONTHREADED = 1 << 10,
    FFTW_NO_BUFFERING = 1 << 11,
    FFTW_NO_INDIRECT_OP = 1 << 12,
    FFTW_ALLOW_LARGE_GENERIC = 1 << 13, /* NO_LARGE_GENERIC is default */
    FFTW_NO_RANK_SPLITS = 1 << 14,
    FFTW_NO_VRANK_SPLITS = 1 << 15,
    FFTW_NO_VRECURSE = 1 << 16,
    FFTW_NO_SIMD = 1 << 17,
    FFTW_NO_SLOW = 1 << 18,
    FFTW_NO_FIXED_RADIX_LARGE_N = 1 << 19,
    FFTW_ALLOW_PRUNING = 1 << 20,
}

#[derive(Debug)]
pub struct Plan<'a, 'b, A>
    where A: 'a + 'b
{
    pub input: &'a mut [A],
    pub output: &'b mut [A],
    plan: ffi::fftw_plan,
}

impl<'a, 'b, A> Plan<'a, 'b, A>
    where A: 'a + 'b
{
    pub fn execute(&self) {
        unsafe {
            ffi::fftw_execute(self.plan);
        }
    }
}

impl<'a, 'b> Plan<'a, 'b, f64> {
    pub fn r2r_1d(in_: &'a mut [f64], out: &'b mut [f64], kind: R2R_KIND, flag: FLAG) -> Self {
        let n = in_.len();
        let plan = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  in_.as_mut_ptr(),
                                  out.as_mut_ptr(),
                                  kind,
                                  flag as u32)
        };
        Plan {
            input: in_,
            output: out,
            plan: plan,
        }
    }
}
