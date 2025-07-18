module simple_clk_rst_generator (
    output logic clk,
    output logic rst
);
    initial begin
        clk = 0;
        forever #5 clk = ~clk; // 100MHz clock
    end

    initial begin
        rst = 1;
        #20 rst = 0;
    end
endmodule
