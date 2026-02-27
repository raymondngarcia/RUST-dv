mod bring_up_tests {
    use rust_axi_tb::*;

    #[test]
    fn test_axi_basic() -> Result<(), Box<dyn std::error::Error>> {
        // Optionally, set TEST environment variable
        std::env::set_var("TEST", "test_axi_basic");
        // Run your simulation
        run_test();
        Ok(())
    }

    #[test]
    fn test_axi_basi_2() -> Result<(), Box<dyn std::error::Error>> {
        // Optionally, set TEST environment variable
        std::env::set_var("TEST", "test_axi_basic_2");
        // Run your simulation
        //run_test();
        Ok(())
    }
}
