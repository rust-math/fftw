#[allow(unused_imports)]
use super::*;

#[test]
fn bindgen_test_layout_fftw_iodim() {
    assert_eq!(
        ::std::mem::size_of::<fftw_iodim>(),
        12usize,
        concat!("Size of: ", stringify!(fftw_iodim))
    );
    assert_eq!(
        ::std::mem::align_of::<fftw_iodim>(),
        4usize,
        concat!("Alignment of ", stringify!(fftw_iodim))
    );
    assert_eq!(
        unsafe { &(*(0 as *const fftw_iodim)).n as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(fftw_iodim),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const fftw_iodim)).is as *const _ as usize },
        4usize,
        concat!(
            "Alignment of field: ",
            stringify!(fftw_iodim),
            "::",
            stringify!(is)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const fftw_iodim)).os as *const _ as usize },
        8usize,
        concat!(
            "Alignment of field: ",
            stringify!(fftw_iodim),
            "::",
            stringify!(os)
        )
    );
}

#[test]
fn bindgen_test_layout_fftw_iodim64() {
    assert_eq!(
        ::std::mem::size_of::<fftw_iodim64>(),
        24usize,
        concat!("Size of: ", stringify!(fftw_iodim64))
    );
    assert_eq!(
        ::std::mem::align_of::<fftw_iodim64>(),
        8usize,
        concat!("Alignment of ", stringify!(fftw_iodim64))
    );
    assert_eq!(
        unsafe { &(*(0 as *const fftw_iodim64)).n as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(fftw_iodim64),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const fftw_iodim64)).is as *const _ as usize },
        8usize,
        concat!(
            "Alignment of field: ",
            stringify!(fftw_iodim64),
            "::",
            stringify!(is)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const fftw_iodim64)).os as *const _ as usize },
        16usize,
        concat!(
            "Alignment of field: ",
            stringify!(fftw_iodim64),
            "::",
            stringify!(os)
        )
    );
}
