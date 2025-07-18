use std::{
    env,
    ffi::CString,
    fs::{create_dir_all, File},
    os::unix::prelude::AsRawFd,
};

extern "C" {
    fn sim_init(enable_wave: bool, vcd_path: *const std::os::raw::c_char);
    fn sim_tick(time: u64, step: u64);
    fn sim_finish();
}

/// Safe wrapper around the Verilator simulation interface
pub struct Sim {
    time: u64,
    step: u64,
}

impl Sim {
    pub fn new(enable_wave: bool, vcd_path: &str, step: u64) -> Self {
        let cstr = CString::new(vcd_path).unwrap();
        unsafe {
            sim_init(enable_wave, cstr.as_ptr());
        }
        Sim { time: 0, step }
    }

    pub fn tick(&mut self) {
        unsafe {
            sim_tick(self.time, self.step);
        }
        self.time += self.step;
    }
}

impl Drop for Sim {
    fn drop(&mut self) {
        unsafe {
            sim_finish();
        }
    }
}

/// Redirect stdout and stderr to the specified file
fn redirect_stdio(log_file: &File) {
    let fd = log_file.as_raw_fd();
    unsafe {
        libc::dup2(fd, libc::STDOUT_FILENO);
        libc::dup2(fd, libc::STDERR_FILENO);
    }
}

pub fn run_test() {
    let test_name = env::var("TEST_NAME").unwrap_or_else(|_| "unnamed_test".to_string());
    let sim_dir = format!("sim/{}", test_name);
    let vcd_path = format!("{}/waveform.vcd", sim_dir);
    let log_path = format!("{}/sim.log", sim_dir);

    create_dir_all(&sim_dir).expect("failed to create sim dir");

    let log_file = File::create(&log_path).expect("failed to create sim.log");
    redirect_stdio(&log_file);

    let mut sim = Sim::new(true, &vcd_path, 10); // step = 10

    for _ in 0..1000 {
        sim.tick();
    }

    println!("Simulation completed for {}", test_name);
}
