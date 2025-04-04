use crate::aux::check_only_vars;
use crate::aux::expresion_eval::{ExprNode, ExpressionEvaluator};
use crate::boolean_operations::BooleanOperations;
use crate::truth_table::generate_truth_table;

impl ExpressionEvaluator<bool, BooleanOperations> {
    pub fn conjunctive_normal_form(&mut self, formula: &str, truth_table: Option<bool>) -> String {
        if truth_table.unwrap() == true {
            self.derive_cnf_from_truth_table(formula).unwrap()
        } else {
            let tree: ExprNode<bool> = self
                .build_tree(formula, check_only_vars(formula))
                .expect("Failed to build tree");

            // Convert the tree to NNF then distribute it to form cnf
            let nnf_tree = self.to_nnf(tree);
            let cnf_tree = self.to_cnf(nnf_tree);

            self.to_rpn(&cnf_tree)
        }
    }

    pub fn derive_cnf_from_truth_table(&mut self, formula: &str) -> Result<String, String> {
        // Generate the truth table
        let truth_table = generate_truth_table(formula, self)?;

        // Collect clauses for rows where the formula evaluates to false
        let mut clauses: Vec<String> = Vec::new();

        for (assignment, result) in truth_table.rows {
            if !result {
                // Create a clause for this row
                let mut clause: Vec<String> = Vec::new();

                for (var, value) in truth_table.variables.iter().zip(assignment) {
                    if value {
                        // If the variable is true, add its negation
                        clause.push(format!("{}!", var));
                    } else {
                        // If the variable is false, add the variable itself
                        clause.push(var.to_string());
                    }
                }

                // Combine the literals in the clause with the correct number of | operators
                let mut rpn_clause = clause.join("");
                for _ in 1..clause.len() {
                    rpn_clause.push('|');
                }

                clauses.push(rpn_clause);
            }
        }
        /*
            If the formula is a tautology, return it as is
            https://en.wikipedia.org/wiki/Tautology_(logic)
        */
        if clauses.is_empty() {
            return Ok(formula.to_string());
        }
        // Combine all clauses with AND
        let mut cnf = clauses.join("");
        for _ in 1..clauses.len() {
            cnf.push('&');
        }

        Ok(cnf)
    }

    fn to_cnf(&self, node: ExprNode<bool>) -> ExprNode<bool> {
        match node {
            // Base cases: constants, variables, and negations
            ExprNode::Const(_) | ExprNode::Var(_) | ExprNode::UnaryOp(_, _) => node,

            // Handle binary operators
            ExprNode::BinaryOp(op, left, right) => {
                let left_cnf = self.to_cnf(*left);
                let right_cnf = self.to_cnf(*right);

                match op {
                    '&' => {
                        // Flatten nested conjunctions into a single top-level conjunction
                        self.flatten_conjunction(left_cnf, right_cnf)
                    }
                    '|' => {
                        // Flatten nested disjunctions into a single top-level disjunction
                        self.flatten_disjunction(left_cnf, right_cnf)
                    }
                    _ => ExprNode::BinaryOp(op, Box::new(left_cnf), Box::new(right_cnf)),
                }
            }
        }
    }

    // Helper function to flatten conjunctions
    fn flatten_conjunction(&self, left: ExprNode<bool>, right: ExprNode<bool>) -> ExprNode<bool> {
        match (left, right) {
            // If both sides are conjunctions, merge them
            (
                ExprNode::BinaryOp('&', left_and, right_and),
                ExprNode::BinaryOp('&', left_and2, right_and2),
            ) => ExprNode::BinaryOp(
                '&',
                Box::new(self.flatten_conjunction(*left_and, *right_and)),
                Box::new(self.flatten_conjunction(*left_and2, *right_and2)),
            ),
            // If one side is a conjunction, merge the other side into it
            (ExprNode::BinaryOp('&', left_and, right_and), other)
            | (other, ExprNode::BinaryOp('&', left_and, right_and)) => ExprNode::BinaryOp(
                '&',
                Box::new(self.flatten_conjunction(*left_and, *right_and)),
                Box::new(other),
            ),
            // If neither side is a conjunction, create a new conjunction
            (left, right) => ExprNode::BinaryOp('&', Box::new(left), Box::new(right)),
        }
    }

    // Helper function to flatten disjunctions
    fn flatten_disjunction(&self, left: ExprNode<bool>, right: ExprNode<bool>) -> ExprNode<bool> {
        match (left, right) {
            // If both sides are disjunctions, merge them
            (
                ExprNode::BinaryOp('|', left_or, right_or),
                ExprNode::BinaryOp('|', left_or2, right_or2),
            ) => ExprNode::BinaryOp(
                '|',
                Box::new(self.flatten_disjunction(*left_or, *right_or)),
                Box::new(self.flatten_disjunction(*left_or2, *right_or2)),
            ),
            // If one side is a disjunction, merge the other side into it
            (ExprNode::BinaryOp('|', left_or, right_or), other)
            | (other, ExprNode::BinaryOp('|', left_or, right_or)) => ExprNode::BinaryOp(
                '|',
                Box::new(self.flatten_disjunction(*left_or, *right_or)),
                Box::new(other),
            ),
            // If neither side is a disjunction, create a new disjunction
            (left, right) => ExprNode::BinaryOp('|', Box::new(left), Box::new(right)),
        }
    }
}

pub fn run_conjunctive_normal_form() {
    let mut boolean_evaluation: ExpressionEvaluator<bool, BooleanOperations> =
        ExpressionEvaluator::<bool, BooleanOperations>::new();
    println!("\n\tRunning conjunctive_normal_form function\n");
    let formula = "ABCD&|&";
    println!("Original formula: {}", formula);
    let cnf = boolean_evaluation.conjunctive_normal_form(formula, Some(true));
    println!("Conjunctive Normal Form {}", cnf);
    println!(
        "Formula {}",
        boolean_evaluation.print_formula(
            &boolean_evaluation
                .build_tree(&cnf, check_only_vars(&cnf))
                .unwrap()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_not_truth() {
        let mut boolean_evaluation: ExpressionEvaluator<bool, BooleanOperations> =
            ExpressionEvaluator::<bool, BooleanOperations>::new();

        /*
        I'm implementing it using the truth table method
        The subject is implementing it using the tree method
        The subject's implementation is more efficient
        The truth table method is easier to implement since I already have the truth table function
        */
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("A", Some(false)),
            "A"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AA&AA&&", Some(false)),
            "AA&AA&&"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB&!", Some(false)),
            "A!B!|"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|!", Some(false)),
            "A!B!&"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|C&", Some(false)),
            "AB|C&"
        );
        /*
         The subject writes it as ABC||| but that's just moving the formula from left associative to right associative
         (((A | B) | C) | D) => (A | (B | (C | D)))
         That's not what CNF is about, CNF is about distributing OR over AND
        */
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|C|D|", Some(false)),
            "AB|C|D|"
        );
        /*
        Formula of AB|C&! is (A or B) and (not C)
        The CNF of this formula is C!A!|C!B!|&
        Which is equivalent to (not C or not A) and (not C or not B)
        */
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|C&!", Some(false)),
            "A!B!&C!|"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("ABCD&|&", Some(false)),
            "ABCD&|&"
        );
        /*
        The subject writes it as ABCD&&& but that's just moving the formula from left associative to right associative
        (((A & B) & C) & D) => (A & (B & (C & D)))
        That's not what CNF is about, CNF is about distributing OR over AND
        */
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB&C&D&", Some(false)),
            "AB&C&D&"
        );
        /*
        Formula for AB&!C!| is (A and B) or (not C)
        The CNF of this formula is A!B!|C!|
            Which is equivalent to (not A or not B) or (not C)
            Subject gives us A!B!C!||
            Which is equivalent to (not A) or (not B or not C)
            */
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB&!C!|", Some(false)),
            "A!B!|C!|"
        );
        /*
        Formula AB|!C!&  is (A or B) and (not C), subject is again just moving the association to the right, which is not CNF
        */
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|!C!&", Some(false)),
            "A!B!&C!&"
        );
    }
    #[test]
    fn test_second_method_with_truth_table() {
        let mut boolean_evaluation: ExpressionEvaluator<bool, BooleanOperations> =
            ExpressionEvaluator::<bool, BooleanOperations>::new();

        /*
            I'm implementing it using the truth table method
            The subject is implementing it using the tree method
            The subject's implementation is more efficient
            The truth table method is easier to implement since I already have the truth table function
        */
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("A", Some(true)),
            "A"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AA&AA&&", Some(true)),
            "A"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB&!", Some(true)),
            "A!B!|"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|!", Some(true)),
            "AB!|A!B|A!B!|&&"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|C&", Some(true)),
            "ABC||ABC!||AB!C||A!BC||A!B!C||&&&&"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|C|D|", Some(true)),
            "ABCD|||"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|C&!", Some(true)),
            "AB!C!||A!BC!||A!B!C!||&&"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("ABCD&|&", Some(true)),
            "ABCD|||ABCD!|||ABC!D|||ABC!D!|||AB!CD|||AB!CD!|||AB!C!D|||AB!C!D!|||A!BCD|||A!BCD!|||A!BC!D|||&&&&&&&&&&"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB&C&D&", Some(true)),
            "ABCD|||ABCD!|||ABC!D|||ABC!D!|||AB!CD|||AB!CD!|||AB!C!D|||AB!C!D!|||A!BCD|||A!BCD!|||A!BC!D|||A!BC!D!|||A!B!CD|||A!B!CD!|||A!B!C!D|||&&&&&&&&&&&&&&"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB&!C!|", Some(true)),
            "A!B!C!||"
        );
        assert_eq!(
            boolean_evaluation.conjunctive_normal_form("AB|!C!&", Some(true)),
            "ABC!||AB!C||AB!C!||A!BC||A!BC!||A!B!C||A!B!C!||&&&&&&"
        );
    }
    #[test]
    fn test_result_truth_table() {
        let mut evaluator: ExpressionEvaluator<bool, BooleanOperations> =
            ExpressionEvaluator::<bool, BooleanOperations>::new();
        let formula = "AB|!C!&";
        let cnf = evaluator.conjunctive_normal_form(formula, Some(true));
        assert_eq!(
            generate_truth_table(formula, &mut evaluator).unwrap(),
            generate_truth_table(&cnf, &mut evaluator).unwrap()
        );
    }
}
