# ===========================
# Makefile for Verilator + Rust
# ===========================

# Path to the Verilator static library that will be built
SIM_LIB = obj_dir/libvltick.a

# Name of the Rust test to run
# The "?=" means "only set this if TEST is not already set on the command line"
# Example: make run TEST=test_axi_write
TEST ?= test_axi_basic

# ---------------------------
# Default target: build everything
# ---------------------------
all:
	# Step 1: Run Verilator to:
	#   -Wall        : enable all warnings
	#   --cc         : generate C++ simulator
	#   --trace      : enable waveform tracing
	#   rtl/...v     : specify the Verilog RTL file(s)
	#   sim/wrapper.cpp : optional C++ testbench wrapper
	#   --Mdir obj_dir  : put generated files into obj_dir
	#   -CFLAGS -Iinclude : include paths for C++ compiler
	#   --top-module simple_axi_slv : specify the top module
	verilator -Wall --cc --trace rtl/simple_axi_slv/simple_axi_slv.v \
	    sim/wrapper.cpp \
	    --Mdir obj_dir -CFLAGS -Iinclude \
	    --top-module simple_axi_slv

	# Step 2: Build the Verilator-generated C++ simulation
	# This uses the Makefile that Verilator created inside obj_dir
	make -C obj_dir -f Vsimple_axi_slv.mk

	# Step 3: Archive all .o object files into a static library
	# This allows Rust to link easily to the simulation
	ar rcs $(SIM_LIB) obj_dir/*.o


# ---------------------------
# Run the Rust testbench
# ---------------------------
run:
	# Step 1: Create a timestamp for this test run
	# Step 2: Combine the test name and timestamp into TEST_NAME
	# Step 3: Run cargo test for the specified Rust test
	# Step 4: Show output in terminal AND save it to sim/last_run.log
	@TEST_TS=$$(date +%Y%m%d_%H%M) && \
	TEST_NAME=$${TEST}_$${TEST_TS} && \
	cargo test --test $(TEST) -- --nocapture 2>&1 | tee sim/last_run.log


# ---------------------------
# Clean all generated files
# ---------------------------
clean:
	# Remove:
	#   - Verilator build directory (obj_dir)
	#   - Rust build artifacts (target)
	#   - Old simulation logs for this test
	#   - Last run log
	rm -rf obj_dir target sim/$(TEST)* sim/last_run.log
