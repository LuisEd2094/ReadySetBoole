use ready_set_boole::adder::run_adder;
use ready_set_boole::boolean_operations::run_boolean_operations;
use ready_set_boole::grey_code::run_grey_code;
use ready_set_boole::multiplier::run_multiplier;
use ready_set_boole::boolean_operations::nnf::run_negation_normal_form;
use ready_set_boole::boolean_operations::cnf::run_conjunctive_normal_form;
use ready_set_boole::truth_table::run_truth_table;
use ready_set_boole::boolean_operations::sat::run_sat_truth_table;
use ready_set_boole::sets::powerset::run_powerset;
use ready_set_boole::sets::eval_set::run_set_operations;
use ready_set_boole::curve::map::run_map;
use ready_set_boole::curve::unmap::run_reverse_map;
// Runs simple tests for each exercise, you can run 'cargo test' for a few more tests
fn main() {
    run_adder();
    run_multiplier();
    run_grey_code();
    run_boolean_operations();
    run_truth_table();
    run_negation_normal_form();
    run_conjunctive_normal_form();
    run_sat_truth_table();
    run_powerset();
    run_set_operations();
    run_map();
    run_reverse_map();
}
