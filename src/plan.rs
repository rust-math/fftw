
use ffi;
use super::r2r::*;
use super::FLAG;

#[derive(Debug)]
pub struct Plan<'a, 'b, A>
    where A: 'a + 'b
{
    pub field: &'a mut [A],
    pub coef: &'b mut [A],
    logical_size: usize,
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
    pub fn r2r_1d(field: &'a mut [f64], coef: &'b mut [f64], kind: R2R_KIND, flag: FLAG) -> Self {
        let n = field.len();
        let forward = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  field.as_mut_ptr(),
                                  coef.as_mut_ptr(),
                                  forward(kind),
                                  flag as u32)
        };
        let backward = unsafe {
            ffi::fftw_plan_r2r_1d(n as i32,
                                  coef.as_mut_ptr(),
                                  field.as_mut_ptr(),
                                  backward(kind),
                                  flag as u32)
        };
        Plan {
            field: field,
            coef: coef,
            logical_size: logical_size(n, kind),
            forward: forward,
            backward: backward,
        }
    }
}
