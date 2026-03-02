// tests/bring_up_tests.rs
use data_types::bits::Bits;
use scheduler::Scheduler;
use sim::VerilatorSim;

#[test]
fn test_axi_basic_1() -> Result<(), Box<dyn std::error::Error>> {
    // Optionally set a TEST environment variable for logging or selection
    std::env::set_var("TEST", "test_axi_basic");

    // -------- 1️⃣ Initialize the Verilator backend --------
    let mut sim = VerilatorSim::new("test_axi_basic");

    // -------- 2️⃣ Create scheduler and attach components --------
    let mut sched = Scheduler::new();

    // Example: add your TB components (drivers, monitors) here
    // sched.add_component(Box::new(MyAxiDriver::new()));
    // sched.add_component(Box::new(MyAxiMonitor::new()));

    // -------- 3️⃣ Run simulation for N cycles --------
    sched.run(&mut sim, 100); // 100 cycles
    sim.log("Simulation finished");

    Ok(())
}

#[test]
fn test_axi_basic_2() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("TEST", "test_axi_basic_2");

    let mut sim = VerilatorSim::new("test_axi_basic_2");
    let mut sched = Scheduler::new();

    // Add components if needed
    // sched.add_component(Box::new(MyAxiDriver::new()));

    sched.run(&mut sim, 50); // shorter run
    sim.log("Simulation finished");
    Ok(())
}

#[test]
fn test_bits() {
    let mut a = Bits::<5>::new(0b10101);
    let sim = VerilatorSim::new("test_bits");

    assert_eq!(a.get(0), true);
    assert_eq!(a.get(1), false);
    a.set(1, true);
    assert_eq!(a.value(), 0b10111);

    let b = Bits::<12>::new(0xABC);
    assert_eq!(b.value(), 0xABC);

    let c = Bits::<100>::new(0xFFFF_FFFF);
    assert_eq!(c.get(0), true);
    assert_eq!(c.get(31), true);
    sim.log("Simulation finished");
}
