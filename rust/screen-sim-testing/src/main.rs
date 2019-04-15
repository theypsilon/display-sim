use screen_sim_testing::fake::FakeVideoInput;

fn main() -> Result<(), String> {
    println!("Running 1.000.000.000.000.000 iterations!!\nTip: Better stop it at some point manually ;)");
    FakeVideoInput::default().iterate_times(1_000_000_000_000_000).map_err(|e| format!("{:?}", e))
}
