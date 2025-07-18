#include "Vsimple_axi_slv.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

Vsimple_axi_slv* dut = nullptr;
VerilatedVcdC* tfp = nullptr;

extern "C" void sim_init(bool enable_wave, const char* dumpfile) {
    Verilated::traceEverOn(enable_wave);
    dut = new Vsimple_axi_slv;

    if (enable_wave) {
        tfp = new VerilatedVcdC;
        dut->trace(tfp, 99);
        tfp->open(dumpfile);
    }
}

extern "C" void sim_tick(uint64_t time) {
    dut->clk = 0;
    dut->eval();
    if (tfp) tfp->dump(time);

    dut->clk = 1;
    dut->eval();
    if (tfp) tfp->dump(time + 5);
}

extern "C" void sim_finish() {
    if (tfp) tfp->close();
    delete tfp;
    delete dut;
}
