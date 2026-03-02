use std::fs;
use std::process::Command;

fn main() {
    let sim_lib = "obj_dir/libvltick.a";

    // Run Verilator
    let status = Command::new("verilator")
        .args([
            "-Wall",
            "--cc",
            "--trace",
            "rtl/simple_axi_slv/simple_axi_slv.v",
            "sim/wrapper.cpp",
            "--Mdir",
            "obj_dir",
            "-CFLAGS",
            "-Iinclude",
            "--top-module",
            "simple_axi_slv",
        ])
        .status()
        .expect("Failed to run Verilator");
    assert!(status.success(), "Verilator failed");

    // Build Verilator-generated simulation
    let status = Command::new("make")
        .args(["-C", "obj_dir", "-f", "Vsimple_axi_slv.mk"])
        .status()
        .expect("Failed to run make in obj_dir");
    assert!(status.success(), "Verilator make failed");

    // Step 3: Archive all .o files manually
    let mut objects = vec![];
    for entry in fs::read_dir("obj_dir").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|s| s == "o").unwrap_or(false) {
            objects.push(path.to_string_lossy().to_string());
        }
    }

    if !objects.is_empty() {
        let mut cmd = Command::new("ar");
        cmd.arg("rcs").arg(sim_lib);
        cmd.args(&objects);
        let status = cmd.status().expect("Failed to run ar");
        assert!(status.success(), "ar failed");
    } else {
        panic!("No .o files found in obj_dir to archive!");
    }

    println!("cargo:rerun-if-changed=rtl/simple_axi_slv/simple_axi_slv.v");
    println!("cargo:rerun-if-changed=sim/wrapper.cpp");

    println!("cargo:rustc-link-search=native=obj_dir");
    println!("cargo:rustc-link-lib=static=vltick");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
