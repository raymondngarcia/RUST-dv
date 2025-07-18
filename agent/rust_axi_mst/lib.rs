extern "C" {
  fn simulate_axi_slv(cycles: i32) -> i32;
}

pub fn run_test() {
  let result = unsafe { simulate_axi_slv(20) };
  println!("Simulation returned {}.", result);
}
