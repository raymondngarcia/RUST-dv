#include "Vsimple_axi_slv.h"
#include "verilated.h"
#include "verilated_vcd_c.h"
#include <cstdint>

struct SimHandle {
    Vsimple_axi_slv* dut;
    VerilatedVcdC* tfp;
};

extern "C" SimHandle* sim_init(bool enable_wave, const char* dumpfile) {
    // Step 1: Enable tracing globally BEFORE DUT is created
    if (enable_wave) {
        Verilated::traceEverOn(true);
    }

    // Step 2: Create DUT
    auto* h = new SimHandle();
    h->dut = new Vsimple_axi_slv;

    // Step 3: Allocate VCD trace AFTER DUT is constructed
    if (enable_wave) {
        h->tfp = new VerilatedVcdC;
        h->dut->trace(h->tfp, 99);
        h->tfp->open(dumpfile);
    } else {
        h->tfp = nullptr;
    }

    return h;
}

extern "C" void sim_tick(SimHandle* h, uint64_t time, uint64_t step) {
    h->dut->clk = 0;
    h->dut->eval();
    if (h->tfp) h->tfp->dump(time);

    h->dut->clk = 1;
    h->dut->eval();
    if (h->tfp) h->tfp->dump(time + (step / 2));
}

extern "C" void sim_finish(SimHandle* h) {
    if (h->tfp) h->tfp->close();
    delete h->tfp;
    delete h->dut;
    delete h;
}
