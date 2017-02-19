
extern crate bindgen;

use bindgen::builder;

fn main() {
    let bindings = builder()
        .header("/usr/include/fftw3.h")
        .whitelisted_function("fftw.*")
        .whitelisted_type("fftw.*")
        .no_unstable_rust()
        .generate()
        .unwrap();
    bindings.write_to_file("src/fftw3.rs").unwrap();
}
