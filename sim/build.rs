use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Cargo output dir
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let verilator_out = out_dir.join("verilator");
    fs::create_dir_all(&verilator_out).unwrap();

    // Absolute paths for RTL and wrapper.cpp
    let rtl_path = fs::canonicalize("../rtl/simple_axi_slv/simple_axi_slv.v")
        .expect("Failed to locate RTL file");
    let wrapper_cpp =
        fs::canonicalize("../sim/cpp/wrapper.cpp").expect("Failed to locate wrapper.cpp");

    // -------- 1️⃣ Run Verilator --------
    let status = Command::new("verilator")
        .args(["-Wall", "--cc", "--trace"])
        .arg(rtl_path)
        .arg(wrapper_cpp)
        .args(["--Mdir"])
        .arg(&verilator_out)
        .args([
            "-CFLAGS",
            "-Iinclude", // adjust include path if needed
            "--top-module",
            "simple_axi_slv",
        ])
        .status()
        .expect("Failed to run Verilator");

    assert!(status.success(), "Verilator failed");

    // -------- 2️⃣ Build generated Makefile --------
    let status = Command::new("make")
        .arg("-C")
        .arg(&verilator_out)
        .arg("-f")
        .arg("Vsimple_axi_slv.mk")
        .status()
        .expect("Failed to run make");

    assert!(status.success(), "Verilator make failed");

    // -------- 3️⃣ Collect object files --------
    let mut objects = Vec::new();
    for entry in fs::read_dir(&verilator_out).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|s| s == "o").unwrap_or(false) {
            objects.push(path);
        }
    }

    if objects.is_empty() {
        panic!("No object files produced by Verilator!");
    }

    // -------- 4️⃣ Archive into static lib --------
    let sim_lib = verilator_out.join("libvltick.a");
    let mut ar = Command::new("ar");
    ar.arg("rcs").arg(&sim_lib);
    for obj in &objects {
        ar.arg(obj);
    }
    let status = ar.status().expect("Failed to run ar");
    assert!(status.success(), "ar failed");

    // -------- 5️⃣ Cargo rebuild triggers --------
    println!("cargo:rerun-if-changed=../rtl/simple_axi_slv/simple_axi_slv.v");
    println!("cargo:rerun-if-changed=../sim/cpp/wrapper.cpp");

    // -------- 6️⃣ Link instructions --------
    println!("cargo:rustc-link-search=native={}", verilator_out.display());
    println!("cargo:rustc-link-lib=static=vltick");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
