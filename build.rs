use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("Making M4RI");
    println!("PWD: {:?}", env::current_dir().unwrap());

    if !Path::new("vendor/m4ri/Makefile").exists() {
        println!("Configuring build!");

        let status = Command::new("/usr/bin/autoreconf")
            .arg("--install")
            .current_dir("vendor/m4ri")
            .status()
            .expect("Failed to execute");
        if !status.success() {
            panic!("Autoconf failed!");
        }

        let status = Command::new("./configure")
            .arg("--enable-static")
            .arg("--enable-thread-safe")
            .arg("--disable-png")
            .env("CFLAGS", "-O3 -fPIC")
            .env("CC", "clang")
            .current_dir("vendor/m4ri")
            .status()
            .expect("Failed to execute configure");
        if !status.success() {
            panic!("Configure failed");
        }
    }

    println!("Executing Make");

    let status = Command::new("/usr/bin/make")
        .arg("-j3")
        .current_dir("vendor/m4ri")
        .status()
        .expect("Failed to execute make");
    if !status.success() {
        panic!("Make failed");
    }

    // Output settings for Cargo
    println!("cargo:rustc-link-search=native=vendor/m4ri/.libs");
    println!("cargo:rustc-link-lib=static=m4ri");
}
