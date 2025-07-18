module simple_axi_slv (
    input  logic        clk,
    input  logic        rst,
    output logic [3:0]  debug_counter
);
    logic [3:0] counter;
    always_ff @(posedge clk or posedge rst) begin
        if (rst)
            counter <= 0;
        else
            counter <= counter + 1;
    end
    assign debug_counter = counter;
endmodule