#![allow(non_camel_case_types)]

#[cfg(feature = "source")]
extern crate fftw_src;

#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src;

extern crate libc;
extern crate num_complex;

mod test;

use libc::FILE;
pub use num_complex::Complex32 as fftwf_complex;
pub use num_complex::Complex64 as fftw_complex;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum fftw_r2r_kind {
    FFTW_R2HC = 0,
    FFTW_HC2R = 1,
    FFTW_DHT = 2,
    FFTW_REDFT00 = 3,
    FFTW_REDFT01 = 4,
    FFTW_REDFT10 = 5,
    FFTW_REDFT11 = 6,
    FFTW_RODFT00 = 7,
    FFTW_RODFT01 = 8,
    FFTW_RODFT10 = 9,
    FFTW_RODFT11 = 10,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fftw_iodim {
    pub n: ::std::os::raw::c_int,
    pub is: ::std::os::raw::c_int,
    pub os: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fftw_iodim64 {
    pub n: isize,
    pub is: isize,
    pub os: isize,
}

pub type fftw_write_char_func = ::std::option::Option<
    unsafe extern "C" fn(c: ::std::os::raw::c_char,
                         arg1: *mut ::std::os::raw::c_void),
>;
pub type fftw_read_char_func = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)
                         -> ::std::os::raw::c_int,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fftw_plan_s([u8; 0]);
pub type fftw_plan = *mut fftw_plan_s;

#[link(name = "fftw3")]
extern "C" {
    pub fn fftw_execute(p: fftw_plan);
    pub fn fftw_plan_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_many_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftw_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        ri: *mut f64,
        ii: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru64_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru64_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        ri: *mut f64,
        ii: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_execute_dft(p: fftw_plan, in_: *mut fftw_complex, out: *mut fftw_complex);
    pub fn fftw_execute_split_dft(p: fftw_plan, ri: *mut f64, ii: *mut f64, ro: *mut f64, io: *mut f64);
    pub fn fftw_plan_many_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f64,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftw_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_r2c_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_r2c_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_r2c_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_many_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f64,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_c2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_c2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_dft_c2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        ri: *mut f64,
        ii: *mut f64,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru64_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru64_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru64_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru64_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        ri: *mut f64,
        ii: *mut f64,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_execute_dft_r2c(p: fftw_plan, in_: *mut f64, out: *mut fftw_complex);
    pub fn fftw_execute_dft_c2r(p: fftw_plan, in_: *mut fftw_complex, out: *mut f64);
    pub fn fftw_execute_split_dft_r2c(p: fftw_plan, in_: *mut f64, ro: *mut f64, io: *mut f64);
    pub fn fftw_execute_split_dft_c2r(p: fftw_plan, ri: *mut f64, ii: *mut f64, out: *mut f64);
    pub fn fftw_plan_many_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f64,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f64,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_r2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind: fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_r2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind0: fftw_r2r_kind,
        kind1: fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_r2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind0: fftw_r2r_kind,
        kind1: fftw_r2r_kind,
        kind2: fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut f64,
        out: *mut f64,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_plan_guru64_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut f64,
        out: *mut f64,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
    pub fn fftw_execute_r2r(p: fftw_plan, in_: *mut f64, out: *mut f64);
    pub fn fftw_destroy_plan(p: fftw_plan);
    pub fn fftw_forget_wisdom();
    pub fn fftw_cleanup();
    pub fn fftw_set_timelimit(t: f64);
    pub fn fftw_plan_with_nthreads(nthreads: ::std::os::raw::c_int);
    pub fn fftw_init_threads() -> ::std::os::raw::c_int;
    pub fn fftw_cleanup_threads();
    pub fn fftw_make_planner_thread_safe();
    pub fn fftw_export_wisdom_to_filename(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn fftw_export_wisdom_to_file(output_file: *mut FILE);
    pub fn fftw_export_wisdom_to_string() -> *mut ::std::os::raw::c_char;
    pub fn fftw_export_wisdom(write_char: fftw_write_char_func, data: *mut ::std::os::raw::c_void);
    pub fn fftw_import_system_wisdom() -> ::std::os::raw::c_int;
    pub fn fftw_import_wisdom_from_filename(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn fftw_import_wisdom_from_file(input_file: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fftw_import_wisdom_from_string(input_string: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn fftw_import_wisdom(
        read_char: fftw_read_char_func,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn fftw_fprint_plan(p: fftw_plan, output_file: *mut FILE);
    pub fn fftw_print_plan(p: fftw_plan);
    pub fn fftw_sprint_plan(p: fftw_plan) -> *mut ::std::os::raw::c_char;
    pub fn fftw_malloc(n: usize) -> *mut ::std::os::raw::c_void;
    pub fn fftw_alloc_real(n: usize) -> *mut f64;
    pub fn fftw_alloc_complex(n: usize) -> *mut fftw_complex;
    pub fn fftw_free(p: *mut ::std::os::raw::c_void);
    pub fn fftw_flops(p: fftw_plan, add: *mut f64, mul: *mut f64, fmas: *mut f64);
    pub fn fftw_estimate_cost(p: fftw_plan) -> f64;
    pub fn fftw_cost(p: fftw_plan) -> f64;
    pub fn fftw_alignment_of(p: *mut f64) -> ::std::os::raw::c_int;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fftwf_plan_s([u8; 0]);
pub type fftwf_plan = *mut fftwf_plan_s;
pub type fftwf_iodim = fftw_iodim;
pub type fftwf_iodim64 = fftw_iodim64;
pub use self::fftw_r2r_kind as fftwf_r2r_kind;
pub type fftwf_write_char_func = fftw_write_char_func;
pub type fftwf_read_char_func = fftw_read_char_func;

#[link(name = "fftw3f")]
extern "C" {
    pub fn fftwf_execute(p: fftwf_plan);
    pub fn fftwf_plan_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_many_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftwf_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        ri: *mut f32,
        ii: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru64_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru64_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        ri: *mut f32,
        ii: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_execute_dft(p: fftwf_plan, in_: *mut fftwf_complex, out: *mut fftwf_complex);
    pub fn fftwf_execute_split_dft(p: fftwf_plan, ri: *mut f32, ii: *mut f32, ro: *mut f32, io: *mut f32);
    pub fn fftwf_plan_many_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f32,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftwf_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_r2c_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_r2c_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_r2c_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_many_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f32,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_c2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_c2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_dft_c2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        ri: *mut f32,
        ii: *mut f32,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru64_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru64_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru64_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru64_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        ri: *mut f32,
        ii: *mut f32,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_execute_dft_r2c(p: fftwf_plan, in_: *mut f32, out: *mut fftwf_complex);
    pub fn fftwf_execute_dft_c2r(p: fftwf_plan, in_: *mut fftwf_complex, out: *mut f32);
    pub fn fftwf_execute_split_dft_r2c(p: fftwf_plan, in_: *mut f32, ro: *mut f32, io: *mut f32);
    pub fn fftwf_execute_split_dft_c2r(p: fftwf_plan, ri: *mut f32, ii: *mut f32, out: *mut f32);
    pub fn fftwf_plan_many_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f32,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f32,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_r2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind: fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_r2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind0: fftwf_r2r_kind,
        kind1: fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_r2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind0: fftwf_r2r_kind,
        kind1: fftwf_r2r_kind,
        kind2: fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut f32,
        out: *mut f32,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_plan_guru64_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut f32,
        out: *mut f32,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
    pub fn fftwf_execute_r2r(p: fftwf_plan, in_: *mut f32, out: *mut f32);
    pub fn fftwf_destroy_plan(p: fftwf_plan);
    pub fn fftwf_forget_wisdom();
    pub fn fftwf_cleanup();
    pub fn fftwf_set_timelimit(t: f64);
    pub fn fftwf_plan_with_nthreads(nthreads: ::std::os::raw::c_int);
    pub fn fftwf_init_threads() -> ::std::os::raw::c_int;
    pub fn fftwf_cleanup_threads();
    pub fn fftwf_make_planner_thread_safe();
    pub fn fftwf_export_wisdom_to_filename(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn fftwf_export_wisdom_to_file(output_file: *mut FILE);
    pub fn fftwf_export_wisdom_to_string() -> *mut ::std::os::raw::c_char;
    pub fn fftwf_export_wisdom(write_char: fftwf_write_char_func, data: *mut ::std::os::raw::c_void);
    pub fn fftwf_import_system_wisdom() -> ::std::os::raw::c_int;
    pub fn fftwf_import_wisdom_from_filename(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn fftwf_import_wisdom_from_file(input_file: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fftwf_import_wisdom_from_string(input_string: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn fftwf_import_wisdom(
        read_char: fftwf_read_char_func,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn fftwf_fprint_plan(p: fftwf_plan, output_file: *mut FILE);
    pub fn fftwf_print_plan(p: fftwf_plan);
    pub fn fftwf_sprint_plan(p: fftwf_plan) -> *mut ::std::os::raw::c_char;
    pub fn fftwf_malloc(n: usize) -> *mut ::std::os::raw::c_void;
    pub fn fftwf_alloc_real(n: usize) -> *mut f32;
    pub fn fftwf_alloc_complex(n: usize) -> *mut fftwf_complex;
    pub fn fftwf_free(p: *mut ::std::os::raw::c_void);
    pub fn fftwf_flops(p: fftwf_plan, add: *mut f64, mul: *mut f64, fmas: *mut f64);
    pub fn fftwf_estimate_cost(p: fftwf_plan) -> f64;
    pub fn fftwf_cost(p: fftwf_plan) -> f64;
    pub fn fftwf_alignment_of(p: *mut f32) -> ::std::os::raw::c_int;
}
