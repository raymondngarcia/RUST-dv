# Verilator + Rust Testbench

This project demonstrates how to:

- Simulate a simple Verilog AXI slave
- Drive it using Rust through FFI
- Run simulation using Verilator

## Structure

- `rtl/`: Verilog DUT modules
- `sim/`: Verilator C++ harness
- `env/`: Build integration
- `agent/`: Rust agent logic
- `test/`: Rust tests

## Build

```sh

make

```

## Run Test

```sh

cargo test

```
