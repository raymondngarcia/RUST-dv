#include "Vsimple_axi_slv.h"
#include "verilated.h"
#include "verilated_vcd_c.h"
#include <iostream>
#include <filesystem>
#include <string>
#include <chrono>
#include <ctime>
#include <sstream>
#include <stdint.h>

std::string get_timestamped_dir(const std::string& prefix) {
    auto now = std::chrono::system_clock::now();
    std::time_t time = std::chrono::system_clock::to_time_t(now);
    std::tm* gmt = std::localtime(&time);

    std::ostringstream oss;
    oss << prefix;
    oss << std::put_time(gmt, "%Y%m%d_%H%M%S");
    return oss.str();
}

extern "C" int simulate_axi_slv(int cycles) {
    Verilated::traceEverOn(true);

    std::string test_dir = get_timestamped_dir("sim/test_axi_basic_");
    std::filesystem::create_directories(test_dir);

    Vsimple_axi_slv* dut = new Vsimple_axi_slv;
    VerilatedVcdC* tfp = new VerilatedVcdC;
    dut->trace(tfp, 99);
    tfp->open((test_dir + "/waveform.vcd").c_str());

    if (!std::freopen((test_dir + "/sim.log").c_str(), "w", stdout)) {
        std::cerr << "Failed to redirect stdout to log file" << std::endl;
    }

    for (int i = 0; i < cycles; ++i) {
        dut->clk = 0;
        dut->eval();
        tfp->dump(i * 10);

        dut->clk = 1;
        dut->eval();
        tfp->dump(i * 10 + 5);

        std::cout << "Cycle " << i << ": counter = "
                  << (int)(dut->debug_counter) << std::endl;
    }

    int result = dut->debug_counter;
    tfp->close();
    delete tfp;
    delete dut;
    return result;
}
