use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let num_jobs = env::var("NUM_JOBS").unwrap();
    let isl_dir_path = Path::new("isl/");
    let pwd_at_build_start_path = env::current_dir().unwrap();

    if !isl_dir_path.is_dir() {
        panic!(concat!("`isl/` directory not found. Most likely",
                       " `git submdoule update --init --recursive` was not invoked."));
    }

    if !Path::new("isl/imath").is_dir() {
        panic!(concat!("`isl/imath/` directory not found. Most likely",
                       " `git submdoule update --init --recursive` was not invoked."));
    }

    // Goto isl/ before building anything
    // (Not ideal but better than `make` emitting intermediary files into tree)
    env::set_current_dir(&isl_dir_path).expect("Could not cd into isl/");

    if !Path::new("config.status").is_file() {
        Command::new("./autogen.sh").status()
                                    .expect("failed to autoreconf!");
        Command::new("./configure").args(["--prefix", out_dir.as_str()])
                                   .status()
                                   .expect("failed to configure!");
    }

    Command::new("make").args(&["-j", num_jobs.as_str()])
                        .status()
                        .expect("failed to make!");
    Command::new("make").args(&["install"])
                        .status()
                        .expect("failed to make install!");

    // Go back to old_pwd
    env::set_current_dir(pwd_at_build_start_path).expect("Could not cd back into OUT_DIR");

    let isl_lib_dir = Path::new(out_dir.as_str()).join("lib/");

    println!("cargo:rustc-link-lib=isl");
    println!("cargo:rustc-link-search=native={}",
             isl_lib_dir.to_str().unwrap());
}
