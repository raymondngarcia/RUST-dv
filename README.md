# Rust + Verilator Testbench

This project demonstrates a fully working RTL simulation environment using:

- 🦀 Rust for test logic and FFI control
- 🧠 Verilator for RTL simulation
- 📦 Safe abstractions around unsafe C bindings
- 📂 Structured logs and waveform outputs per test

---

## ✅ Features

- Safe Rust wrapper for Verilator simulation (minimal `unsafe`)
- Waveform `.vcd` dumping enabled (`--trace`)
- Per-test output directory: `sim/<test_name>_<timestamp>/`
- Log redirection to `sim.log`
- Fully CLI-driven: `make all` and `make run TEST=<name>`
- Supports multiple tests using standard Rust `#[test]`

---

## 📁 Directory Structure

.

├── agent/rust_axi_mst/         # Rust AXI master test agent + FFI binding

│   └── lib.rs                  # Main simulation interface

├── rtl/simple_axi_slv/         # Verilog DUT (AXI slave)

├── sim/                        # Simulation wrapper and tick logic

│   └── wrapper.cpp             # Clock/eval/VCD dumping logic

├── test/                       # Rust tests

│   └── test_axi_basic.rs

├── build.rs                    # Optional build script (if needed)

├── Cargo.toml                  # Rust workspace

├── Makefile                    # Verilator + test runner

└── README.md                   # This file

## ▶️ How to Run

### 1. Build DUT and simulator

```bash
make all
```

### 2. Run a test

```bash
make run TEST=test_axi_basic
```

This will:

* Run the Rust test
* Redirect all output to `sim/test_axi_basic_<timestamp>/sim.log`
* Dump waveform to `waveform.vcd` in the same folder

### 🧪 Output Example

├── sim/test_axi_basic_20250718_2130/

│   └── sim.log

│   └── waveform.vcd

### ⚙️ **Clock + Eval Logic**

The actual clocking is handled in C++ via Verilator:

```cpp
void sim_tick(uint64_t time, uint64_t step) {
    dut->clk = 0; dut->eval(); tfp->dump(time);
    dut->clk = 1; dut->eval(); tfp->dump(time + step / 2);
}
```

In Rust:

```rust
let mut sim = Sim::new(true, &vcd_path, 10);
for _ in 0..1000 {
    sim.tick();
}
```

### 💡 Safe Rust Abstraction

All FFI to `sim_init`, `sim_tick`, and `sim_finish` is encapsulated safely inside a `Sim` struct. Log redirection is handled via a helper:

```rust
fn redirect_stdio(log_file: &File) {
    unsafe {
        libc::dup2(log_file.as_raw_fd(), libc::STDOUT_FILENO);
        libc::dup2(log_file.as_raw_fd(), libc::STDERR_FILENO);
    }
}
```

### 🧼 Cleanup

```make
make clean
```

Deletes:

* Verilator `obj_dir/`
* Cargo `target/`
* `sim/` logs and waveform folders

### 🧽 Format Code

```bash
cargo fmt
```
