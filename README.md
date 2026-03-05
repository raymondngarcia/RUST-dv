
# Advantages of RUST-dv: A Rust-based Verification Framework

**RUST-dv** is an exploratory, open-source framework for functional verification of register-transfer level (RTL) designs. It uses the Rust programming language to implement testbenches, stimulus generators, drivers, monitors, and scoreboards, with Verilator serving as the initial simulation backend. While the project remains under active development and should be considered a prototype rather than a production-ready solution, it aims to address several longstanding challenges in conventional verification flows.

This document outlines the primary motivations and architectural characteristics of RUST-dv in comparison to traditional approaches based on **SystemVerilog + UVM** and **Python-based environments** (most notably Cocotb).

## 1. Simulation Performance and Regression Throughput

Simulation speed is frequently the dominant bottleneck in verification campaigns, especially when pursuing high coverage goals or running large constrained-random regressions.

- **Commercial event-driven simulators** (VCS, Questa, Xcelium, etc.) used with UVM provide mature support for the full SystemVerilog language and advanced verification features, but their interpretive/event-driven nature introduces non-trivial runtime overhead.
- **Python-based frameworks** such as Cocotb incur additional performance penalties due to frequent crossings between the Python runtime and the simulator via VPI, DPI, or FLI interfaces. This overhead becomes particularly noticeable in cycle-accurate stimulus/response patterns or when performing many signal accesses per cycle.

By contrast, **Verilator** compiles synthesizable Verilog/SystemVerilog into highly optimized, statically linked C++ code, delivering cycle-accurate simulation performance that is often substantially faster — frequently 10–100× — than interpreted simulators on many designs.

In the current RUST-dv implementation, testbench logic written in Rust links directly to the Verilator-generated model with minimal overhead, enabling significantly higher regression throughput during design bring-up and coverage collection phases.

**Important architectural note**: While Verilator is used as the reference simulator during early development, the scheduler, context abstractions, and component interfaces have been intentionally designed to remain simulator-agnostic. This separation is intended to enable future support for other simulation backends (including event-driven commercial simulators and mixed-language environments) without requiring major rewrites of existing testbench components.

## 2. Memory Safety and Concurrency Correctness

Verification environments are notoriously prone to subtle concurrency bugs (races between driving and sampling, clock-domain mismatches, scoreboard corruption) and memory-related defects that can produce misleading results or intermittent failures.

- **SystemVerilog** offers powerful object-oriented and constrained-randomization facilities, but provides no compile-time guarantees against data races, use-after-free errors, or invalid memory accesses.
- **Python-based testbenches** inherit dynamic typing and garbage collection, deferring many classes of errors (type mismatches, invalid handles, signal name typos) until runtime — often after hours or days of simulation.

Rust’s core language features — ownership, borrowing, lifetimes, and strict threading rules — eliminate these defect classes at compile time with zero runtime cost. In RUST-dv, the central scheduler enforces a strict phased execution model (typically Drive → Evaluate → Sample), and all interactions between components occur through a borrow-checked `Context` object. This design substantially reduces the likelihood of testbench-induced bugs that have historically consumed large amounts of debug effort.

## 3. Tooling, Reproducibility, and Developer Experience

Contemporary software development emphasizes fast feedback, reproducibility, and seamless integration with continuous integration pipelines — qualities that are often difficult to achieve with legacy hardware verification flows.

- UVM environments commonly depend on vendor-specific build scripts, complex environment variable setups, license-managed compute farms, and non-trivial debug/replay mechanisms.
- Cocotb improves ergonomics through Python, but remains coupled to the simulator’s native build process and is limited by Python’s Global Interpreter Lock (GIL) for true concurrency.

RUST-dv leverages **Cargo**, Rust’s integrated build system and package manager, to provide:

- Single-command test invocation (`cargo test`) with built-in parallelism across independent test cases
- Automatic per-test isolation of logs, waveform files, coverage data, and artifacts in timestamped output directories
- Fully reproducible builds without reliance on external environment variables or per-seat licenses

These characteristics bring verification workflows closer to modern software engineering practices and enable efficient execution of large regression suites on commodity hardware or cloud CI runners.

## 4. Modularity and Reusability

The framework defines trait-based interfaces for drivers, monitors, scoreboards, sequencers, and stimulus generators, taking advantage of Rust’s zero-cost abstractions, generics, and compile-time polymorphism.

This contrasts with:

- UVM’s class-based factory and virtual method dispatch model (which incurs runtime overhead)
- Cocotb’s coroutine-based approach (which can introduce runtime indirection)

Although the current codebase contains only basic examples (e.g., AXI slave verification), the separation of the scheduler into its own crate facilitates incremental development of reusable verification component libraries.

## Current Limitations and Development Status

RUST-dv is an early-stage research-oriented prototype with several important limitations:

- Only **Verilator** is currently supported as a simulation backend.
- Waveform dumping requires single-test execution to avoid file-access conflicts.
- The framework does not yet provide mature libraries for constrained randomization, functional coverage collection, temporal assertions, or a broad set of industry-standard VIPs.
- Documentation, example breadth, and test suite coverage remain limited.

These factors position RUST-dv as an experimental platform rather than a direct replacement for established industrial flows. It may be most appropriate for:

- New block-level designs
- Open-source hardware projects
- Teams interested in exploring safer and higher-performance verification paradigms

## Summary

RUST-dv combines Rust’s strong safety guarantees and modern tooling ecosystem with Verilator’s compilation-based simulation performance to pursue three principal goals:

1. Substantially higher regression throughput
2. Significant reduction in testbench-induced defects
3. Improved developer experience and alignment with contemporary software practices

Although currently limited to Verilator and still maturing, the framework’s modular architecture establishes a foundation for potential future support of additional simulators and incremental expansion of verification capabilities.

This direction contributes to broader research into verification methodologies that more effectively leverage advances in programming language design, type systems, and compiler technology.

---


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
.

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




