use crate::boolean_operations::BooleanOperations;
use std::collections::HashSet;

pub struct TruthTable {
    pub variables: Vec<char>,
    pub rows: Vec<(Vec<bool>, bool)>,
}

pub fn print_truth_table(table: &TruthTable) {
    print!("| ");
    for var in &table.variables {
        print!("{} | ", var);
    }
    println!("= |");

    println!("{}", "-".repeat(4 * (table.variables.len() + 1)));

    for (assignment, result) in &table.rows {
        print!("| ");
        for &value in assignment {
            print!("{} | ", if value { 1 } else { 0 });
        }
        println!("{} |", if *result { 1 } else { 0 });
    }
}

pub fn generate_truth_table(
    formula: &str,
    evaluator: &mut BooleanOperations,
) -> Result<TruthTable, String> {
    let mut variables: Vec<char> = formula
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    variables.sort();

    let num_vars: usize = variables.len();
    let mut rows: Vec<(Vec<bool>, bool)> = Vec::new();
    let permutations: usize = 1 << num_vars;

    // Generate all 2^n combinations
    for i in 0..permutations {
        let mut assignment: Vec<bool> = vec![false; num_vars];
        let mut formula_instance: String = formula.to_string();
        for (j, var) in variables.iter().enumerate() {
            /*
            Each bit of i represents the value of a variable in the formula.
            For example if i = 3, thats 011 in binary,
            so with three variables A, B, C, the assignment would be [false, true, true]
            we move the bits to the right by j and check if the LSB is 1 to know if the variable is true or false.

            0	000	0	0	0
            1	001	1	0	0
            2	010	0	1	0
            3	011	1	1	0
            4	100	0	0	1
            5	101	1	0	1
            6	110	0	1	1
            7	111	1	1	1
            */
            let value = ((i >> j) & 1) == 1;
            assignment[j] = value;
            formula_instance = formula_instance.replace(*var, if value { "1" } else { "0" });
        }

        // Evaluate the formula with current variable assignments
        match evaluator.evaluate(&formula_instance) {
            Ok(result) => rows.push((assignment, result)),
            Err(err) => {
                return Err(format!(
                    "Error evaluating formula '{}': {}",
                    formula_instance, err
                ))
            }
        }
    }
    rows.sort();
    Ok(TruthTable { variables, rows })
}

pub fn run_truth_table() {
    println!("\n\tRunning truth table function\n");

    let mut boolean_operations: BooleanOperations = BooleanOperations::new();

    let expression = "AB&C|";
    //let expression = "AB&CD&EF&GH&IJ&KL&MN&OP&QR&ST&UV&&&&&&&&&&&";
    match generate_truth_table(expression, &mut boolean_operations) {
        Ok(table) => print_truth_table(&table),
        Err(err) => eprintln!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_and() {
        let mut evaluator = BooleanOperations::new();
        let formula = "AB&";
        let truth_table = generate_truth_table(formula, &mut evaluator).unwrap();

        let expected = vec![
            (vec![false, false], false),
            (vec![false, true], false),
            (vec![true, false], false),
            (vec![true, true], true),
        ];

        assert_eq!(truth_table.rows, expected);
    }

    #[test]
    fn test_simple_or() {
        let mut evaluator = BooleanOperations::new();
        let formula = "AB|"; // A ∨ B
        let truth_table = generate_truth_table(formula, &mut evaluator).unwrap();

        let expected = vec![
            (vec![false, false], false),
            (vec![false, true], true),
            (vec![true, false], true),
            (vec![true, true], true),
        ];

        assert_eq!(truth_table.rows, expected);
    }

    #[test]
    fn test_negation() {
        let mut evaluator = BooleanOperations::new();
        let formula = "A!";
        let truth_table = generate_truth_table(formula, &mut evaluator).unwrap();

        let expected = vec![(vec![false], true), (vec![true], false)];

        assert_eq!(truth_table.rows, expected);
    }

    #[test]
    fn test_complex_expression() {
        let mut evaluator = BooleanOperations::new();
        let formula = "AB&C|";
        let truth_table = generate_truth_table(formula, &mut evaluator).unwrap();

        let expected = vec![
            (vec![false, false, false], false),
            (vec![false, false, true], true),
            (vec![false, true, false], false),
            (vec![false, true, true], true),
            (vec![true, false, false], false),
            (vec![true, false, true], true),
            (vec![true, true, false], true),
            (vec![true, true, true], true),
        ];

        assert_eq!(truth_table.rows, expected);
    }

    #[test]
    fn test_invalid_formula() {
        let mut evaluator = BooleanOperations::new();
        let formula = "AB!!";

        let result = generate_truth_table(formula, &mut evaluator);
        assert!(result.is_err());
    }
}
