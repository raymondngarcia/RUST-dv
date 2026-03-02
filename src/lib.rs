use chrono::Local;
use std::{
    env,
    ffi::CString,
    fs::{create_dir_all, File},
    io::{self, Write},
};
//use BITS::*;

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

/// Logger that writes only to a file, ignoring stdout
pub struct FileOnly {
    file: File,
}

impl Write for FileOnly {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

/// Run a simulation test, log to sim.log and print only a final line to terminal
pub fn run_test() {
    let test_name = env::var("TEST").unwrap_or_else(|_| "unnamed_test".to_string());
    let timestamp = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
    let sim_dir = format!("sim/{}_{}", test_name, timestamp);
    let vcd_path = format!("{}/waveform.vcd", sim_dir);
    let log_path = format!("{}/sim.log", sim_dir);

    create_dir_all(&sim_dir).expect("failed to create sim dir");

    let file = File::create(&log_path).expect("failed to create sim.log");
    let mut logger = FileOnly { file }; // logs everything only to sim.log

    let mut sim = Sim::new(true, &vcd_path, 10); // step = 10

    for _ in 0..1000 {
        sim.tick();
        //writeln!(logger, "Tick at time {}", sim.time).unwrap();
    }

    writeln!(logger, "Simulation completed for {}", test_name).unwrap();

    // Only print final line to terminal for Cargo
    println!("Simulation completed for {}", test_name);
}
