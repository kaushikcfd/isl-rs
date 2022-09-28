use std::process::Command;
use std::env;
use std::path::Path;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let num_jobs = env::var("NUM_JOBS").unwrap();
    let isl_dir_path = Path::new("isl/");
    let pwd_at_build_start_path = env::current_dir().unwrap();

    if !isl_dir_path.is_dir() {
        panic!("isl directory not found. Most likely you missed calling `git submdoule update --init`.");
    }

    if !Path::new("isl/imath").is_dir() {
        panic!("isl directory not found. Most likely you missed calling `git submdoule update --init --recursive`.");
    }

    // Got to isl/ before building anything
    if !env::set_current_dir(&isl_dir_path).is_ok() {
        panic!("Could not cd into isl/")
    }

    Command::new("./autogen.sh")
        .status()
        .expect("failed to autoreconf!");
    Command::new("./configure")
        .args(["--prefix", out_dir.as_str()])
        .status()
        .expect("failed to configure!");
    Command::new("make")
        .args(&["-j", num_jobs.as_str()])
        .status()
        .expect("failed to make!");
    Command::new("make")
        .args(&["install"])
        .status()
        .expect("failed to make install!");

    // Got to out_dir/ before building anything
    if !env::set_current_dir(pwd_at_build_start_path).is_ok() {
        panic!("Could not cd back into OUT_DIR")
    }

    let isl_lib_dir = Path::new(out_dir.as_str()).join("lib/");

    println!("cargo:rustc-link-lib=isl");
    println!("cargo:rustc-link-search=native={}", isl_lib_dir.to_str().unwrap());
}
