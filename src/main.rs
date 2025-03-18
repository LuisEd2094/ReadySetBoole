use ready_set_boole::adder::run_adder;
use ready_set_boole::boolean_evaluation::run_boolean_operations;
use ready_set_boole::grey_code::run_grey_code;
use ready_set_boole::multiplier::run_multiplier;
use ready_set_boole::truth_table::run_truth_table;

// Runs simple tests for each exercise, you can run 'cargo test' for a few more tests

fn main() {
    run_adder();
    run_multiplier();
    run_grey_code();
    run_boolean_operations();
    run_truth_table();
}
