use std::env::var;
use std::path::*;
use std::process::Command;

fn build_fftw(flags: &[&str], src_dir: &Path, out_dir: &Path) {
    run(Command::new("./configure").args(flags).current_dir(
        &src_dir,
    ));
    run(
        Command::new("make")
            .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
            .current_dir(&src_dir),
    );
    run(
        Command::new("make")
            .arg("install")
            .arg(format!("DESTDIR={}", out_dir.display()))
            .current_dir(&src_dir),
    );
}

fn main() {
    let root = PathBuf::from(".");
    let source = PathBuf::from("fftw-3.3.6-pl1");
    let output = PathBuf::from(var("OUT_DIR").unwrap());

    if !source.exists() {
        run(
            Command::new("wget")
                .arg("http://www.fftw.org/fftw-3.3.6-pl1.tar.gz")
                .current_dir(&root),
        );
        run(
            Command::new("tar")
                .args(&["zxvf", "fftw-3.3.6-pl1.tar.gz"])
                .current_dir(&root),
        );
    }

    build_fftw(&["--enable-shared", "--enable-single"], &source, &output);
    build_fftw(&["--enable-shared"], &source, &output);

    println!(
        "cargo:rustc-link-search={}",
        output.join("usr/local/lib").display()
    );

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
