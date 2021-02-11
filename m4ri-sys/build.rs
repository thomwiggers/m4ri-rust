use std::{env, path::PathBuf};
use std::{io, fs};
use std::path::Path;
use std::process::Command;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Making M4RI");
    println!("PWD: {:?}", env::current_dir().unwrap());
    let mut out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
    out_dir.push("m4ri");
    println!("OUT_DIR = {:?}", &out_dir);

    copy_dir_all("vendor/m4ri", &out_dir)?;

    if !out_dir.join("Makefile").exists() {
        println!("cargo:info=Building m4ri lib");
        println!("Configuring build!");

        let status = Command::new("autoreconf")
            .arg("--install")
            .current_dir(&out_dir)
            .status()
            .expect("Failed to execute autoreconf");
        if !status.success() {
            panic!("Autoconf failed!");
        }

        let status = Command::new("./configure")
            .arg("--enable-static")
            .arg("--enable-thread-safe")
            .arg("--disable-png")
            .env("CFLAGS", "-Ofast -fPIC")
            .current_dir(&out_dir)
            .status()
            .expect("Failed to execute ./configure");
        if !status.success() {
            panic!("Configure failed");
        }
    }

    println!("Executing Make");

    let status = Command::new("make")
        .arg("-j5")
        .current_dir(&out_dir)
        .status()
        .expect("Failed to execute /usr/bin/make");
    if !status.success() {
        panic!("Make failed");
    }

    // Output settings for Cargo
    println!("cargo:rustc-link-search=native={}", out_dir.join(".libs").to_str().unwrap());
    //println!("cargo:rustc-link-search=native=m4ri-sys/{}", out_dir.join(".libs").to_str().unwrap());
    println!("cargo:rustc-link-lib=static=m4ri");

    Ok(())
}
