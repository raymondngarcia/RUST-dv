# Rust + Verilator Design Verification Framework

This project demonstrates a scalable RTL simulation environment using:

- Rust for test logic, drivers, and VIPs
- Verilator for RTL simulation
- Safe abstractions around unsafe C bindings
- Structured logs and waveform outputs per test under `target`

---

## ✅ Features

- Fully independent **scheduler crate** for driving/sampling phases
- Safe Rust wrapper for Verilator simulation (minimal `unsafe`)
- Waveform `.vcd` dumping enabled via `WAVE=1`
- Per-test output directory: `target/tb/sim_outputs/<test_name>_<timestamp>/`
- Automatic log redirection to `sim.log`
- CLI-driven test selection with Rust `#[test]`
- Test outputs automatically cleaned via `cargo clean`

---

## 📂 Directory Structure
├── data_types/ # Custom Bits<N> types for flexible-width signals
├── rtl/simple_axi_slv/ # Verilog DUT (AXI slave)
├── scheduler/ # Scheduler crate: phases, context, components
├── sim/ # Simulation crate wrapping Verilator
│ └── cpp/
│ └── wrapper.cpp
├── tb/ # Testbench crate: Rust test logic
│ └── tests/
│ └── test_axi_basic.rs
├── target/ # Cargo output, including sim outputs
├── build.rs # Build script for Verilator integration
├── Cargo.toml # Workspace manifest
└── README.md


---

## ▶️ How to Run

### 1. Build simulator

```bash
cargo build -p sim
```

- This runs sim/build.rs which:
    - Compiles Verilator RTL + wrapper.cpp
    - Places generated files under target/sim/obj_dir
    - Produces static library libvltick.a linked by Rust

### 2. Run a test
```bash
cargo test -p tb --test test_axi_basic
```

### 3. Output
- Test outputs are written per test under:

target/tb/sim_outputs/<test_name>_<timestamp>/
├── sim.log
└── wave_<test_name>.vcd  # only if WAVE=1

- Example
target/tb/sim_outputs/test_axi_basic_2026-03-02_21-50-12-123/
├── sim.log
└── wave_test_axi_basic.vcd

## Scheduler API
- Rust Scheduler manages simulation phases:

```rust
scheduler.on_phase(Phase::Drive, &mut ctx);
backend.eval();
scheduler.on_phase(Phase::Sample, &mut ctx);
ctx.advance();
```

## Safe Verilator Wrapper
```rust
let mut sim = VerilatorSim::new(test_name);
sim.log("Starting simulation");
sim.eval();
```

- Encapsulates sim_init, sim_tick, sim_finish
- Logs automatically to sim.log
- Dumps waveforms to unique per-test folder if WAVE=1

## Notes
- Only one test should enable waveforms at a time to avoid SIGSEGV
- Test outputs are isolated per test and timestamped for reproducibility
- cargo clean removes all outputs under target, including simulation logs and waves
- Multiple tests can run in parallel safely, with independent outputs

## Tips
- Run only integration tests if unit tests and doc tests are not desired:
```bash
cargo test --test <test_name>
```




