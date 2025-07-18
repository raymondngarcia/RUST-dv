//use std::{env, ffi::CString, fs::{create_dir_all, File}, io::Write, os::unix::prelude::AsRawFd, path::Path};

use std::{
  env,
  ffi::CString,
  fs::{create_dir_all, File},
  os::unix::prelude::AsRawFd,
};

extern "C" {
    fn sim_init(enable_wave: bool, vcd_path: *const std::os::raw::c_char);
    fn sim_tick(time: u64);
    fn sim_finish();
}

pub fn run_test() {
    // Use test name from environment (set by cargo test)
    let test_name = env::var("TEST_NAME").unwrap_or_else(|_| "unnamed_test".to_string());
    let sim_dir = format!("sim/{}", test_name);
    let vcd_path = format!("{}/waveform.vcd", sim_dir);
    let log_path = format!("{}/sim.log", sim_dir);

    create_dir_all(&sim_dir).expect("failed to create sim dir");

    // Redirect stdout/stderr to sim.log
    let log_file = File::create(&log_path).expect("failed to create sim.log");
    unsafe {
        let fd = log_file.as_raw_fd();
        libc::dup2(fd, libc::STDOUT_FILENO);
        libc::dup2(fd, libc::STDERR_FILENO);
    }

    let vcd_cstr = CString::new(vcd_path).unwrap();

    unsafe {
        sim_init(true, vcd_cstr.as_ptr());

        for cycle in 0..1000 {
            let time = cycle * 10;
            sim_tick(time);
        }

        sim_finish();
    }

    println!("Simulation completed for {}", test_name);
}