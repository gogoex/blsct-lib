use std::env;
use std::path::PathBuf;

fn main() {
    let repo_root = &PathBuf::from("../..");
    let src_dir = &repo_root.join("src");
    let include_dir = &repo_root.join("include");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // -L
    println!("cargo:rustc-link-search={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-search={}", src_dir.to_str().unwrap());
    println!("cargo:rustc-link-search={}/bls/lib", src_dir.to_str().unwrap());
    println!("cargo:rustc-link-search={}/bls/mcl/lib", src_dir.to_str().unwrap());

    // -l
    println!("cargo:rustc-link-lib=blsct");
    println!("cargo:rustc-link-lib=bls384_256");
    println!("cargo:rustc-link-lib=mcl");
    println!("cargo:rustc-link-lib=stdc++");

    // header file to create bindings
    println!("cargo:rerun-if-changed={}/lib.cpp", src_dir.to_str().unwrap());
    println!("cargo:rerun-if-changed={}/lib.h", include_dir.to_str().unwrap());

    if !std::process::Command::new("g++")
        .arg("-c")
        .arg("-I").arg(src_dir.join("bls").join("include"))
        .arg("-I").arg(src_dir.join("bls").join("mcl").join("include"))
        .arg("-I").arg(repo_root)
        .arg("-I").arg(repo_root.join("include"))
        .arg("-o").arg(out_dir.join("blsct.o"))
        .arg(src_dir.join("lib.cpp"))
        .output()
        .expect("could not spawn `g++`")
        .status
        .success()
    {
        panic!("Failed to build lib.o");
    }
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(out_dir.join("libblsct.a"))
        .arg(out_dir.join("blsct.o"))
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        panic!("Failed to build libblsct.a");
    }

    // TODO find out why the first attempt with `.compile("blsct") fails
    // but the second attempt with `.compile("blsct2")` works
    // currently using lower level code above, but better to use cc::Build if possible
    //
    // cc::Build::new()
    //     .cpp(true)
    //     .file(src_dir.join("lib.cpp"))
    //     .include(src_dir.join("bls").join("include"))
    //     .include(src_dir.join("bls").join("mcl").join("include"))
    //     .include(repo_root)
    //     .include(repo_root.join("include"))
    //     .compile("blsct");

    let bindings = bindgen::Builder::default()
        .header(include_dir.join("lib.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to generate bindings");
}