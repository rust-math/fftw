extern crate md5;

use std::env::var;
use std::fs;
use std::io::*;
use std::path::*;
use std::process::Command;

fn download(uri: &str, filename: &str, out_dir: &Path) {
    let out = out_dir.join(filename);
    let mut f = BufWriter::new(fs::File::create(out).unwrap());
    let p = Command::new("curl")
        .arg(uri)
        .output()
        .expect("Failed to start download");
    f.write(&p.stdout).unwrap();
}

fn check_sum(archive: &Path) -> Result<[u8; 16]> {
    let mut f = fs::File::open(archive)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(md5::compute(&buf).into())
}

fn expand(archive: &Path, out_dir: &Path) {
    let st = Command::new("tar")
        .args(&["xvf", archive.to_str().unwrap()])
        .current_dir(&out_dir)
        .status()
        .expect("Failed to start expanding archive");
    if !st.success() {
        panic!("Failed to expand archive");
    }
}

fn build_fftw(flags: &[&str], src_dir: &Path, out_dir: &Path) {
    run(Command::new("./configure")
        .args(flags)
        .current_dir(&src_dir));
    run(Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&src_dir));
    run(Command::new("make")
        .arg("install")
        .arg(format!("DESTDIR={}", out_dir.display()))
        .current_dir(&src_dir));
}

const FFTW: &'static str = "fftw-3.3.6-pl1";
const ARCHIVE: &'static str = "fftw-3.3.6-pl1.tar.gz";
const URI: &'static str = "http://www.fftw.org/fftw-3.3.6-pl1.tar.gz";
const MD5SUM: u128 = 0x682a0e78d6966ca37c7446d4ab4cc2a1;

fn correct_sum() -> [u8; 16] {
    let mut bytes = unsafe { ::std::mem::transmute::<u128, [u8; 16]>(MD5SUM) };
    bytes.reverse();
    bytes
}

fn main() -> Result<()> {
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let archive_path = out_dir.join(ARCHIVE);
    let src_dir = out_dir.join(FFTW);

    if !archive_path.exists() {
        download(URI, ARCHIVE, &out_dir);
    }
    if check_sum(&archive_path)? != correct_sum() {
        panic!("check sum of archive is incorrect");
    }
    expand(&archive_path, &out_dir);

    build_fftw(
        &["--enable-static", "--with-pic", "--enable-single"],
        &src_dir,
        &out_dir,
    );
    build_fftw(&["--enable-static", "--with-pic"], &src_dir, &out_dir);

    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("usr/local/lib").display()
    );

    println!("cargo:rustc-link-lib=static=fftw3");
    println!("cargo:rustc-link-lib=static=fftw3f");

    Ok(())
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
