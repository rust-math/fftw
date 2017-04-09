
use std::env::{remove_var, var};
use std::path::PathBuf;
use std::process::Command;

macro_rules! variable(($name:expr) => (var($name).unwrap()));

fn main() {
    let root = PathBuf::from(".");
    let source = PathBuf::from("fftw-3.3.6-pl1");
    let output = PathBuf::from(variable!("OUT_DIR").replace(r"\", "/"));

    remove_var("TARGET");
    if !source.exists() {
        run(Command::new("wget")
            .arg("http://www.fftw.org/fftw-3.3.6-pl1.tar.gz")
            .current_dir(&root));
        run(Command::new("tar").args(&["zxvf", "fftw-3.3.6-pl1.tar.gz"]).current_dir(&root));
    }
    run(Command::new("./configure").arg("--enable-shared").current_dir(&source));

    run(Command::new("make")
        .arg(format!("-j{}", variable!("NUM_JOBS")))
        .current_dir(&source));

    run(Command::new("make")
        .arg("install")
        .arg(format!("DESTDIR={}", output.display()))
        .current_dir(&source));

    println!("cargo:rustc-link-search={}",
             output.join("usr/local/lib").display());
    println!("cargo:rustc-link-lib=dylib=fftw3");
    println!("cargo:rustc-link-lib=dylib=fftw3f");
}

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => {
            if !status.success() {
                panic!("`{:?}` failed: {}", command, status);
            }
        }
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}
