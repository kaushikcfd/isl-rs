use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let num_jobs = env::var("NUM_JOBS").unwrap();
    let isl_dir_path = Path::new("isl/");
    let pwd_at_build_start_path = env::current_dir().unwrap();
    let mut copy_options = CopyOptions::new();
    copy_options.skip_exist = true;

    if !isl_dir_path.is_dir() {
        panic!(concat!("`isl/` directory not found. Most likely",
                       " `git submdoule update --init --recursive` was not invoked."));
    }

    if !Path::new("isl/imath").is_dir() {
        panic!(concat!("`isl/imath/` directory not found. Most likely",
                       " `git submdoule update --init --recursive` was not invoked."));
    }

    // Copy to out_dir and change isl path
    copy(isl_dir_path, out_dir.to_string().as_str(), &copy_options).unwrap();
    let isl_dir_path = Path::new(&out_dir).join("isl/");

    // Goto isl/ before building anything
    // (Not ideal but better than `make` emitting intermediary files into tree)
    env::set_current_dir(&isl_dir_path).expect("Could not cd into isl/");

    Command::new("./autogen.sh").status()
                                .expect("failed to autoreconf!");
    Command::new("./configure").args(["--prefix",
                                      out_dir.as_str(),
                                      "--with-pic=isl",
                                      "--with-int=imath",
                                      "--enable-shared=no",
                                      "--enable-static=yes"])
                               .status()
                               .expect("failed to configure!");

    Command::new("make").args(&["-j", num_jobs.as_str()])
                        .status()
                        .expect("failed to make!");
    Command::new("make").args(&["install"])
                        .status()
                        .expect("failed to make install!");

    // Go back to old_pwd
    env::set_current_dir(pwd_at_build_start_path).expect("Could not cd back into OUT_DIR");

    println!("cargo:rustc-link-search=native={}/lib/", out_dir);
    println!("cargo:rustc-link-lib=static=isl");
    println!("cargo:rerun-if-changed=isl/");
}
