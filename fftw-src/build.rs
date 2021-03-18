use anyhow::Result;
use std::env::{var, set_var};
use std::fs::{canonicalize, File};
use std::io::{copy, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::ZipArchive;

fn download_archive_windows(out_dir: &Path) -> Result<()> {
    if out_dir.join("libfftw3.dll").exists() && out_dir.join("libfftw3f.dll").exists() {
        return Ok(());
    }

    let archive = out_dir.join("fftw_windows.zip");
    if !archive.exists() {
        // Download
        let mut conn = ftp::FtpStream::connect("ftp.fftw.org:21")?;
        conn.login("anonymous", "anonymous")?;
        conn.cwd("pub/fftw")?;
        let buf = conn.simple_retr("fftw-3.3.5-dll64.zip")?.into_inner();
        // TODO calc checksum
        let mut f = File::create(&archive)?;
        f.write(&buf)?;
    }
    let f = File::open(&archive)?;
    let mut zip = ZipArchive::new(f)?;
    let target = var("TARGET").unwrap();
    for name in &["fftw3-3", "fftw3f-3"] {
        for ext in &["dll", "def"] {
            let filename = format!("lib{}.{}", name, ext);
            let mut zf = zip.by_name(&filename)?;
            let mut f = File::create(out_dir.join(filename))?;
            copy(&mut zf, &mut f)?;
        }
        run(cc::windows_registry::find_tool(&target, "lib.exe")
            .unwrap()
            .to_command()
            .arg("/MACHINE:X64")
            .arg(format!("/DEF:lib{}.def", name))
            .arg(format!("/OUT:lib{}.lib", name))
            .current_dir(out_dir));
    }
    Ok(())
}

fn build_unix(out_dir: &Path, flags: &[&str]) {
    let src_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap()).join("fftw-3.3.8");
    let out_src_dir = out_dir.join("src");
    fs_extra::dir::copy(
        src_dir,
        &out_src_dir,
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: true,
            depth: 0,
            content_only: false,
        },
    )
    .unwrap();
    if !out_dir.join("lib/libfftw3.a").exists() {
        build_fftw(flags, &out_src_dir, &out_dir);
    }
    if !out_dir.join("lib/libfftw3f.a").exists() {
        let mut flags = flags.to_vec();
        flags.push("--enable-single");
        build_fftw(&flags, &out_src_dir, &out_dir);
    }
}

fn build_fftw(flags: &[&str], src_dir: &Path, out_dir: &Path) {
    run(
        Command::new(canonicalize(src_dir.join("configure")).unwrap())
            .arg("--with-pic")
            .arg("--enable-static")
            .arg("--disable-doc")
            .arg(format!("--prefix={}", out_dir.display()))
            .args(flags)
            .current_dir(&src_dir),
    );
    run(Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&src_dir));
    run(Command::new("make").arg("install").current_dir(&src_dir));
}

fn run(command: &mut Command) -> String {
    println!("Running: {:?}", command);
    match command.output() {
        Ok(output) => {
            if !output.status.success() {
                panic!("`{:?}` failed: {}", command, output.status);
            }
            return String::from_utf8(output.stdout).unwrap()
        },
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-cahnged=build.rs");
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let target = var("TARGET").unwrap();
    let triple = target.split("-").collect::<Vec<_>>();
    let target_os = var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = match triple[0] {
        "aarch64" => "arm64",
        "armv7" => "armv7",
        "armv7s" => "armv7s",
        "x86" => "x86",
        "x86_64" => "x86_64",
        &_ => panic!("Unsupported platform {}", target_os),
    };
    match target_os.as_ref() {
        "ios" => {
            let cc = run(Command::new("xcrun").args(&["--sdk", "iphoneos", "-f", "clang"]));
            set_var("CC", cc.trim());
            let sysroot = run(Command::new("xcrun").args(&["--sdk", "iphoneos", "--show-sdk-path"]));
            set_var("CFLAGS", format!("-O3 -pipe -isysroot {} -arch {} -miphoneos-version-min=9.0", sysroot.trim(), arch));
            let flags = &[&format!("--with-sysroot={}", sysroot.trim()), "--host=arm-apple-darwin"];
            build_unix(&out_dir, flags);
        }
        "android" => {
        },
        "windows" => {
            download_archive_windows(&out_dir).unwrap();
            println!("cargo:rustc-link-search={}", out_dir.display());
            println!("cargo:rustc-link-lib=libfftw3-3");
            println!("cargo:rustc-link-lib=libfftw3f-3");
        }
        _ => {
            if var("CARGO_CFG_TARGET_FAMILY").unwrap_or("".to_string()) != "unix" {
                panic!("Unsupported platform {}", target_os);
            }
            build_unix(&out_dir, &[]);
            println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
            println!("cargo:rustc-link-lib=static=fftw3");
            println!("cargo:rustc-link-lib=static=fftw3f");
        }
    }
}
