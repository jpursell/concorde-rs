// use std::env;
// use std::process::Command;

use glob::glob;

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();

    // Command::new("make")
    //     .env("OUT_DIR", &out_dir)
    //     .args(["-C", "src/concorde"])
    //     .output()
    //     .expect("Failed to make");

    // println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-lib=static=concorde");
    // println!("cargo:rerun-if-changed=src/concorde/*");


    let srcs = glob("./src/concorde/**/*.c").unwrap().into_iter().map(|e| e.unwrap().display().to_string()).collect::<Vec<_>>();
    let heads = glob("./src/concorde/**/*.h").unwrap().into_iter().map(|e| e.unwrap().display().to_string()).collect::<Vec<_>>();
    cc::Build::new()
        .files(&srcs)
        .include("./src/concorde/INCLUDE/")
        .compile("_concorde");

    for path in srcs.iter().chain(heads.iter()) {
        println!("cargo:rerun-if-changed={}", path);
    }
}
