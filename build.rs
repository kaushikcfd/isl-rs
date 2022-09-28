use std::process::Command;
use std::env;
use std::path::Path;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let num_jobs = env::var("NUM_JOBS").unwrap();

    if !Path::new("isl/").is_dir() {
        panic!("isl directory not found. Most likely you missed calling `git submdoule update --init`.");
    }

    if !Path::new("isl/imath").is_dir() {
        panic!("isl directory not found. Most likely you missed calling `git submdoule update --init --recursive`.");
    }

    Command::new("isl/autogen.sh")
        .status()
        .expect("failed to autoreconf!");
    Command::new("isl/configure")
        .args(["--prefix", out_dir.as_str()])
        .status()
        .expect("failed to configure!");
    Command::new("make")
        .args(&[
              "-j", num_jobs.as_str()])
        .status()
        .expect("failed to make!");
    Command::new("make")
        .args(&[
              "install"])
        .status()
        .expect("failed to make install!");

    let isl_lib_dir = Path::new(out_dir.as_str()).join("lib/");

    println!("cargo:rustc-link-lib=isl");
    println!("cargo:rustc-link-search=native={}", isl_lib_dir.to_str().unwrap());
}
