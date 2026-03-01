mod bring_up_tests {
    use data_types::bits::Bits;
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

    #[test]
    fn test_bits() {
        let mut a = Bits::<5>::new(0b10101);
        assert_eq!(a.get(0), true);
        assert_eq!(a.get(1), false);
        a.set(1, true);
        assert_eq!(a.value(), 0b10111);

        let b = Bits::<12>::new(0xABC);
        assert_eq!(b.value(), 0xABC);

        let c = Bits::<100>::new(0xFFFF_FFFF);
        assert_eq!(c.get(0), true);
        assert_eq!(c.get(31), true);
    }
}
