#![allow(non_camel_case_types)]

extern crate fftw3_sys as ffi;

pub use ffi::fftw_r2r_kind as R2R_KIND;

fn forward(kind: R2R_KIND) -> R2R_KIND {
    match kind {
        R2R_KIND::FFTW_R2HC => R2R_KIND::FFTW_R2HC,
        R2R_KIND::FFTW_HC2R => R2R_KIND::FFTW_R2HC,
        R2R_KIND::FFTW_DHT => R2R_KIND::FFTW_DHT,
        R2R_KIND::FFTW_REDFT00 => R2R_KIND::FFTW_REDFT00,
        R2R_KIND::FFTW_REDFT01 => R2R_KIND::FFTW_REDFT10,
        R2R_KIND::FFTW_REDFT10 => R2R_KIND::FFTW_REDFT10,
        R2R_KIND::FFTW_REDFT11 => R2R_KIND::FFTW_REDFT11,
        R2R_KIND::FFTW_RODFT00 => R2R_KIND::FFTW_RODFT00,
        R2R_KIND::FFTW_RODFT01 => R2R_KIND::FFTW_RODFT10,
        R2R_KIND::FFTW_RODFT10 => R2R_KIND::FFTW_RODFT10,
        R2R_KIND::FFTW_RODFT11 => R2R_KIND::FFTW_RODFT11,
    }
}

fn backward(kind: R2R_KIND) -> R2R_KIND {
    match kind {
        R2R_KIND::FFTW_R2HC => R2R_KIND::FFTW_HC2R,
        R2R_KIND::FFTW_HC2R => R2R_KIND::FFTW_HC2R,
        R2R_KIND::FFTW_DHT => R2R_KIND::FFTW_DHT,
        R2R_KIND::FFTW_REDFT00 => R2R_KIND::FFTW_REDFT00,
        R2R_KIND::FFTW_REDFT01 => R2R_KIND::FFTW_REDFT01,
        R2R_KIND::FFTW_REDFT10 => R2R_KIND::FFTW_REDFT01,
        R2R_KIND::FFTW_REDFT11 => R2R_KIND::FFTW_REDFT11,
        R2R_KIND::FFTW_RODFT00 => R2R_KIND::FFTW_RODFT00,
        R2R_KIND::FFTW_RODFT01 => R2R_KIND::FFTW_RODFT01,
        R2R_KIND::FFTW_RODFT10 => R2R_KIND::FFTW_RODFT01,
        R2R_KIND::FFTW_RODFT11 => R2R_KIND::FFTW_RODFT11,
    }
}

/// see http://www.fftw.org/fftw3_doc/Real_002dto_002dReal-Transform-Kinds.html
fn logical_size(n: usize, kind: R2R_KIND) -> usize {
    match kind {
        R2R_KIND::FFTW_R2HC => n,
        R2R_KIND::FFTW_HC2R => n,
        R2R_KIND::FFTW_DHT => n,
        R2R_KIND::FFTW_REDFT00 => 2 * (n - 1),
        R2R_KIND::FFTW_REDFT01 => 2 * n,
        R2R_KIND::FFTW_REDFT10 => 2 * n,
        R2R_KIND::FFTW_REDFT11 => 2 * n,
        R2R_KIND::FFTW_RODFT00 => 2 * (n + 1),
        R2R_KIND::FFTW_RODFT01 => 2 * n,
        R2R_KIND::FFTW_RODFT10 => 2 * n,
        R2R_KIND::FFTW_RODFT11 => 2 * n,
    }
}

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
    pub field: &'a mut [A],
    pub coef: &'b mut [A],
    forward: ffi::fftw_plan,
    backward: ffi::fftw_plan,
}

impl<'a, 'b, A> Plan<'a, 'b, A>
    where A: 'a + 'b
{
    /// [field] -> [coef]
    pub fn forward(&self) {
        unsafe {
            ffi::fftw_execute(self.forward);
        }
    }
    /// [field] <- [coef]
    pub fn backward(&self) {
        unsafe {
            ffi::fftw_execute(self.backward);
        }
    }
}

impl<'a, 'b, A> Drop for Plan<'a, 'b, A> {
    fn drop(&mut self) {
        unsafe {
            ffi::fftw_destroy_plan(self.forward);
            ffi::fftw_destroy_plan(self.backward);
        }
    }
}

impl<'a, 'b> Plan<'a, 'b, f64> {
    pub fn r2r_1d(in_: &'a mut [f64], out: &'b mut [f64], kind: R2R_KIND, flag: FLAG) -> Self {
        let n = in_.len();
        let forward = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  in_.as_mut_ptr(),
                                  out.as_mut_ptr(),
                                  forward(kind),
                                  flag as u32)
        };
        let backward = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  out.as_mut_ptr(),
                                  in_.as_mut_ptr(),
                                  backward(kind),
                                  flag as u32)
        };
        Plan {
            field: in_,
            coef: out,
            forward: forward,
            backward: backward,
        }
    }
}
