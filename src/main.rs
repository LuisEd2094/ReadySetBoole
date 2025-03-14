use ready_set_boole::adder::run_adder;
use ready_set_boole::multiplier::run_multiplier;
use ready_set_boole::grey_code::run_grey_code;

// Runs simple tests for each exercise, you can run 'cargo test' for a few more tests
fn main() {
    run_adder();
    run_multiplier();
    run_grey_code();
}
