SIM_LIB = obj_dir/libvltick.a
TEST ?= test_axi_basic

all:
	verilator -Wall --cc --trace rtl/simple_axi_slv/simple_axi_slv.v \
	    sim/tick.cpp \
	    --Mdir obj_dir -CFLAGS -Iinclude \
	    --top-module simple_axi_slv
	make -C obj_dir -f Vsimple_axi_slv.mk
	ar rcs $(SIM_LIB) obj_dir/*.o

run:
	cargo test --test $(TEST) -- --nocapture 2>&1 | tee sim/last_run.log

clean:
	rm -rf obj_dir target sim/test_axi_basic* sim/last_run.log
