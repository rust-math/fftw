use anyhow::Result;
use cc;
use std::env::{set_var, var};
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
        if var("CARGO_CFG_TARGET_ARCH").unwrap().starts_with("armv7") {
            flags.push("--enable-neon");
        }
        build_fftw(&flags, &out_src_dir, &out_dir);
    }
}

fn build_fftw(flags: &[&str], src_dir: &Path, out_dir: &Path) {
    run(
        Command::new(canonicalize(src_dir.join("configure")).unwrap())
            .arg("--with-pic")
            .arg("--enable-static")
            .arg("--disable-doc")
            .arg("--enable-threads")
            .arg("--with-combined-threads")
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
                panic!(
                    "`{:?}` failed: {}\nstdout:\n{}\nstderr:\n{}",
                    command,
                    output.status,
                    unsafe { String::from_utf8_unchecked(output.stdout) },
                    unsafe { String::from_utf8_unchecked(output.stderr) }
                );
            }
            return String::from_utf8(output.stdout).unwrap();
        }
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
            let tool = cc::Build::new()
                .target(&target)
                .flag_if_supported(&format!("-march={}", arch))
                .get_compiler();
            set_var("CC", tool.cc_env());
            set_var("CFLAGS", tool.cflags_env());
            let sysroot =
                run(Command::new("xcrun").args(&["--sdk", "iphoneos", "--show-sdk-path"]));
            let args = &[
                &format!("--with-sysroot={}", sysroot.trim()),
                "--host=arm-apple-darwin",
            ];
            build_unix(&out_dir, args);
            println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
            println!("cargo:rustc-link-lib=static=fftw3");
            println!("cargo:rustc-link-lib=static=fftw3f");
        }
        "android" => {
            let tool = cc::Build::new()
                .target(&target)
                .flag_if_supported("-mfloat-abi=softfp")
                .flag_if_supported("-mfpu=neon")
                .get_compiler();
            let mut cc = Command::new(tool.cc_env())
                .arg("--version")
                .status()
                .ok()
                .and_then(|status| {
                    if status.success() {
                        Some(tool.cc_env())
                    } else {
                        None
                    }
                });
            let ndk_root: PathBuf = var("ANDROID_NDK_ROOT")
                .map_err(|_| var("ANDROID_NDK_HOME"))
                .expect("ndk not found, please set ANDROID_NDK_ROOT to where ndk installed.")
                .into();
            if cc.is_none() {
                let host = var("HOST").unwrap();
                let triple = host.split("-").collect::<Vec<_>>();
                let toolchain = ndk_root
                    .join("toolchains/llvm/prebuilt")
                    .join(&format!("{}-{}", triple[2], triple[0]))
                    .join("bin");
                if !toolchain.exists() {
                    panic!(format!(
                        "Unsupported platform {}, ndk toolchain dose not exists, {}!",
                        host,
                        toolchain.display()
                    ));
                };
                match target.as_str() {
                    "aarch64-linux-android" => {
                        set_var("AR", toolchain.join("aarch64-linux-android-ar"));
                        set_var("AS", toolchain.join("aarch64-linux-android-as"));
                        set_var("LD", toolchain.join("aarch64-linux-android-ld"));
                        set_var("STRIP", toolchain.join("aarch64-linux-android-strip"));
                        set_var("RANLIB", toolchain.join("aarch64-linux-android-ranlib"));
                        cc = Some(
                            toolchain
                                .join("aarch64-linux-android21-clang")
                                .into_os_string(),
                        );
                    }
                    "armv7-linux-androideabi" => {
                        set_var("AR", toolchain.join("arm-linux-androideabi-ar"));
                        set_var("AS", toolchain.join("arm-linux-androideabi-as"));
                        set_var("LD", toolchain.join("arm-linux-androideabi-ld"));
                        set_var("STRIP", toolchain.join("arm-linux-androideabi-strip"));
                        set_var("RANLIB", toolchain.join("arm-linux-androideabi-ranlib"));
                        cc = Some(
                            toolchain
                                .join("armv7a-linux-androideabi21-clang")
                                .into_os_string(),
                        );
                    }
                    &_ => {
                        unimplemented!();
                    }
                };
            };
            set_var("CFLAGS", tool.cflags_env());
            set_var("CC", cc.unwrap());
            let cross = format!("--host={}", target);
            let sysroot = format!(
                "--with-sysroot={}/platforms/android-21/arch-arm",
                ndk_root.display()
            );
            if target.starts_with("arm") {
                build_unix(&out_dir, &[cross.as_str(), sysroot.as_str()]);
            } else {
                build_unix(&out_dir, &[cross.as_str(), sysroot.as_str()]);
            }
            println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
            println!("cargo:rustc-link-lib=static=fftw3");
            println!("cargo:rustc-link-lib=static=fftw3f");
        }
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
