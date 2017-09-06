use std::sync::Mutex;

lazy_static! {
    pub static ref FFTW_MUTEX: Mutex<()> = Mutex::new(());
}
