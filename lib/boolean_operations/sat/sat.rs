use crate::boolean_operations::BooleanOperations;
use crate::truth_table::generate_truth_table;

impl BooleanOperations {
    pub fn sat_truth_table(&mut self, formula: &str) -> bool {
        match generate_truth_table(&formula, self) {
            Ok(table) => table.rows.iter().any(|(_, result)| *result),
            Err(err) => {
                eprintln!("{}", err);
                return false;
            }
        }
    }
}

pub fn run_sat_truth_table(){
    println!("\n\tRunning SAT truth table function\n");
    let formula: &str = "AB&!";
    let mut evaluator: BooleanOperations = BooleanOperations::new();    
    let res: bool = evaluator.sat_truth_table(formula);

    println!("Original formula: {}", formula);
    println!("SAT Truth Table: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sat_truth_table() {
        let mut evaluator = BooleanOperations::new();
        let formula = "AB|";
        let expected = true;
        assert_eq!(evaluator.sat_truth_table(&formula), expected);

        let formula = "AB&";
        let expected = true;
        assert_eq!(evaluator.sat_truth_table(&formula), expected);

        let formula = "AA!&";
        let expected = false;
        assert_eq!(evaluator.sat_truth_table(&formula), expected);

        let formula = "AA^";
        let expected = false;
        assert_eq!(evaluator.sat_truth_table(&formula), expected);
    }
}
