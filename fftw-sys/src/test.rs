#[test]
fn bindgen_test_layout__IO_marker() {
    assert_eq!(::std::mem::size_of::<_IO_marker>(),
               24usize,
               concat!("Size of: ", stringify!(_IO_marker)));
    assert_eq!(::std::mem::align_of::<_IO_marker>(),
               8usize,
               concat!("Alignment of ", stringify!(_IO_marker)));
    assert_eq!(unsafe { &(*(0 as *const _IO_marker))._next as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_marker),
                       "::",
                       stringify!(_next)));
    assert_eq!(unsafe { &(*(0 as *const _IO_marker))._sbuf as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_marker),
                       "::",
                       stringify!(_sbuf)));
    assert_eq!(unsafe { &(*(0 as *const _IO_marker))._pos as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_marker),
                       "::",
                       stringify!(_pos)));
}

#[test]
fn bindgen_test_layout_fftw_iodim_do_not_use_me() {
    assert_eq!(::std::mem::size_of::<fftw_iodim_do_not_use_me>(),
               12usize,
               concat!("Size of: ", stringify!(fftw_iodim_do_not_use_me)));
    assert_eq!(::std::mem::align_of::<fftw_iodim_do_not_use_me>(),
               4usize,
               concat!("Alignment of ", stringify!(fftw_iodim_do_not_use_me)));
    assert_eq!(unsafe { &(*(0 as *const fftw_iodim_do_not_use_me)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(fftw_iodim_do_not_use_me),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const fftw_iodim_do_not_use_me)).is as *const _ as usize },
               4usize,
               concat!("Alignment of field: ",
                       stringify!(fftw_iodim_do_not_use_me),
                       "::",
                       stringify!(is)));
    assert_eq!(unsafe { &(*(0 as *const fftw_iodim_do_not_use_me)).os as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(fftw_iodim_do_not_use_me),
                       "::",
                       stringify!(os)));
}

#[test]
fn bindgen_test_layout_fftw_iodim64_do_not_use_me() {
    assert_eq!(::std::mem::size_of::<fftw_iodim64_do_not_use_me>(),
               24usize,
               concat!("Size of: ", stringify!(fftw_iodim64_do_not_use_me)));
    assert_eq!(::std::mem::align_of::<fftw_iodim64_do_not_use_me>(),
               8usize,
               concat!("Alignment of ", stringify!(fftw_iodim64_do_not_use_me)));
    assert_eq!(unsafe { &(*(0 as *const fftw_iodim64_do_not_use_me)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(fftw_iodim64_do_not_use_me),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const fftw_iodim64_do_not_use_me)).is as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(fftw_iodim64_do_not_use_me),
                       "::",
                       stringify!(is)));
    assert_eq!(unsafe { &(*(0 as *const fftw_iodim64_do_not_use_me)).os as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(fftw_iodim64_do_not_use_me),
                       "::",
                       stringify!(os)));
}

#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(::std::mem::size_of::<_IO_FILE>(),
               216usize,
               concat!("Size of: ", stringify!(_IO_FILE)));
    assert_eq!(::std::mem::align_of::<_IO_FILE>(),
               8usize,
               concat!("Alignment of ", stringify!(_IO_FILE)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._flags as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_flags)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_read_ptr as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_read_ptr)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_read_end as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_read_end)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_read_base as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_read_base)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_write_base as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_write_base)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_write_ptr as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_write_ptr)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_write_end as *const _ as usize },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_write_end)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_buf_base as *const _ as usize },
               56usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_buf_base)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_buf_end as *const _ as usize },
               64usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_buf_end)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_save_base as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_save_base)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_backup_base as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_backup_base)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._IO_save_end as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_IO_save_end)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._markers as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_markers)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._chain as *const _ as usize },
               104usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_chain)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._fileno as *const _ as usize },
               112usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_fileno)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._flags2 as *const _ as usize },
               116usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_flags2)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._old_offset as *const _ as usize },
               120usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_old_offset)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._cur_column as *const _ as usize },
               128usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_cur_column)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._vtable_offset as *const _ as usize },
               130usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_vtable_offset)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._shortbuf as *const _ as usize },
               131usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_shortbuf)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._lock as *const _ as usize },
               136usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_lock)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._offset as *const _ as usize },
               144usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_offset)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE)).__pad1 as *const _ as usize },
               152usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(__pad1)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE)).__pad2 as *const _ as usize },
               160usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(__pad2)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE)).__pad3 as *const _ as usize },
               168usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(__pad3)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE)).__pad4 as *const _ as usize },
               176usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(__pad4)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE)).__pad5 as *const _ as usize },
               184usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(__pad5)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._mode as *const _ as usize },
               192usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_mode)));
    assert_eq!(unsafe { &(*(0 as *const _IO_FILE))._unused2 as *const _ as usize },
               196usize,
               concat!("Alignment of field: ",
                       stringify!(_IO_FILE),
                       "::",
                       stringify!(_unused2)));
}
