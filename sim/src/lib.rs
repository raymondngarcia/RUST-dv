use scheduler::scheduler::SimBackend;
use std::env;
use std::ffi::CString;
use std::fs;
use chrono::Local;
use std::path::PathBuf;

#[repr(C)]
pub struct SimHandle {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn sim_init(enable_wave: bool, dumpfile: *const i8) -> *mut SimHandle;
    fn sim_tick(h: *mut SimHandle, time: u64, step: u64);
    fn sim_finish(h: *mut SimHandle);
}

pub struct VerilatorSim {
    handle: *mut SimHandle,
    time: u64,
    pub log_path: PathBuf,
}

impl VerilatorSim {
    /// Automatically reads WAVE env var and creates unique folder per test
    pub fn new(test_name: &str) -> Self {
        // Timestamp with milliseconds
        let timestamp = Local::now().format("%Y%m%d_%H%M%S_%3f").to_string();

        // Folder: target/sim_outputs/<test_name>_<timestamp>/
        let folder_name = format!("target/sim_outputs/{}_{}", test_name, timestamp);
        fs::create_dir_all(&folder_name).unwrap();

        // Paths for log and VCD inside that folder
        let log_path = PathBuf::from(&folder_name).join("sim.log");
        let vcd_path = PathBuf::from(&folder_name).join(format!("wave_{}.vcd", test_name));

        // Enable waveform if WAVE env var is set
        let enable_wave = env::var("WAVE")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        let dumpfile = CString::new(vcd_path.to_string_lossy().to_string()).unwrap();

        // Initialize Verilator simulation
        let handle = unsafe { sim_init(enable_wave, dumpfile.as_ptr()) };

        Self { handle, time: 0, log_path }
    }

    /// Convenience: write simulation messages
    pub fn log(&self, msg: &str) {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.log_path)
            .unwrap();
        writeln!(f, "{}", msg).unwrap();
    }
}

impl SimBackend for VerilatorSim {
    fn eval(&mut self) {
        unsafe {
            sim_tick(self.handle, self.time, 2);
            self.time += 2;
        }
    }
}

impl Drop for VerilatorSim {
    fn drop(&mut self) {
        unsafe { sim_finish(self.handle) }
    }
}
