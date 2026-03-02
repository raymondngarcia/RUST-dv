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
        // ------------------------------------------------------------
        // 1️⃣ Timestamp with millisecond precision
        // ------------------------------------------------------------
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S-%3f");

        // ------------------------------------------------------------
        // 2️⃣ Locate workspace root safely
        // ------------------------------------------------------------
        let workspace_root = env::var("CARGO_WORKSPACE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                // Fallback for older Cargo versions
                let manifest_dir =
                    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
                manifest_dir
                    .parent()
                    .expect("Failed to determine workspace root")
                    .to_path_buf()
            });

        let target_dir = workspace_root.join("target");

        // ------------------------------------------------------------
        // 3️⃣ Create folder:
        // workspace_root/target/sim_outputs/<test>_<timestamp>/
        // ------------------------------------------------------------
        let folder = target_dir
            .join("sim_outputs")
            .join(format!("{}_{}", test_name, timestamp));

        fs::create_dir_all(&folder).unwrap();

        // ------------------------------------------------------------
        // 4️⃣ Paths for log and VCD
        // ------------------------------------------------------------
        let log_path = folder.join("sim.log");
        let vcd_path = folder.join(format!("wave_{}.vcd", test_name));

        // ------------------------------------------------------------
        // 5️⃣ Enable waveform via WAVE env var
        // ------------------------------------------------------------
        let enable_wave = env::var("WAVE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let dumpfile = CString::new(
            vcd_path.to_string_lossy().to_string()
        ).unwrap();

        // ------------------------------------------------------------
        // 6️⃣ Initialize Verilator
        // ------------------------------------------------------------
        let handle = unsafe { sim_init(enable_wave, dumpfile.as_ptr()) };

        Self {
            handle,
            time: 0,
            log_path,
        }
    }

    /// Write simulation messages
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
