#pragma once
#include "Vsimple_axi_slv.h"

extern "C" int tick(Vsimple_axi_slv* dut, int cycles);

// --- File: agent/rust_axi_mst/lib.rs ---
mod axi_driver;
use std::ffi::c_int;

#[repr(C)]
pub struct Vsimple_axi_slv {
    pub clk: bool,
    pub rst: bool,
    pub debug_counter: u8,
}

extern "C" {
    fn tick(dut: *mut Vsimple_axi_slv, cycles: c_int) -> c_int;
}

pub fn run_test() {
    let mut dut = Vsimple_axi_slv { clk: false, rst: true, debug_counter: 0 };
    let result = unsafe { tick(&mut dut, 20) };
    println!("Simulation returned {} | Final counter: {}", result, dut.debug_counter);
}
