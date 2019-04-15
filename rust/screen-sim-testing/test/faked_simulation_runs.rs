use screen_sim_testing::fake::{FakeVideoInput};

#[test]
fn test_simulation_initializes_fine() {
    assert_eq!(FakeVideoInput::new().iterate_times(0), Ok(()));
}

#[test]
fn test_simulation_run_2_iterations_fine() {
    assert_eq!(FakeVideoInput::new().iterate_times(2), Ok(()));
}