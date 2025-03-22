use crate::boolean_operations::{BooleanOperations, ExprNode};

impl BooleanOperations {
    pub fn conjunctive_normal_form(&self, formula: &str) -> String {
        let tree = self
            .build_tree(formula, Some(true))
            .expect("Failed to build tree");

        // Convert the tree to NNF then distribute it to form cnf
        let nnf_tree = self.to_nnf(tree);
        let cnf_tree = self.to_cnf(nnf_tree);
        
        self.to_rpn(&cnf_tree)
    }

    fn to_cnf(&self, node: ExprNode) -> ExprNode {
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
    fn flatten_conjunction(&self, left: ExprNode, right: ExprNode) -> ExprNode {
        match (left, right) {
            // If both sides are conjunctions, merge them
            (ExprNode::BinaryOp('&', left_and, right_and), ExprNode::BinaryOp('&', left_and2, right_and2)) => {
                ExprNode::BinaryOp(
                    '&',
                    Box::new(self.flatten_conjunction(*left_and, *right_and)),
                    Box::new(self.flatten_conjunction(*left_and2, *right_and2)),
                )
            }
            // If one side is a conjunction, merge the other side into it
            (ExprNode::BinaryOp('&', left_and, right_and), other) | (other, ExprNode::BinaryOp('&', left_and, right_and)) => {
                ExprNode::BinaryOp(
                    '&',
                    Box::new(self.flatten_conjunction(*left_and, *right_and)),
                    Box::new(other),
                )
            }
            // If neither side is a conjunction, create a new conjunction
            (left, right) => ExprNode::BinaryOp('&', Box::new(left), Box::new(right)),
        }
    }
    
    // Helper function to flatten disjunctions
    fn flatten_disjunction(&self, left: ExprNode, right: ExprNode) -> ExprNode {
        match (left, right) {
            // If both sides are disjunctions, merge them
            (ExprNode::BinaryOp('|', left_or, right_or), ExprNode::BinaryOp('|', left_or2, right_or2)) => {
                ExprNode::BinaryOp(
                    '|',
                    Box::new(self.flatten_disjunction(*left_or, *right_or)),
                    Box::new(self.flatten_disjunction(*left_or2, *right_or2)),
                )
            }
            // If one side is a disjunction, merge the other side into it
            (ExprNode::BinaryOp('|', left_or, right_or), other) | (other, ExprNode::BinaryOp('|', left_or, right_or)) => {
                ExprNode::BinaryOp(
                    '|',
                    Box::new(self.flatten_disjunction(*left_or, *right_or)),
                    Box::new(other),
                )
            }
            // If neither side is a disjunction, create a new disjunction
            (left, right) => ExprNode::BinaryOp('|', Box::new(left), Box::new(right)),
        }
    }
}

pub fn run_conjunctive_normal_form() {
    let boolean_evaluation = BooleanOperations::new();
    println!("\n\tRunning conjunctive_normal_form function\n");
    let formula = "AB&!";
    println!("Original formula: {}", formula);
    let cnf = boolean_evaluation.conjunctive_normal_form(formula);
    println!("Conjunctive Normal Form {}", cnf);
    println!("Formula {}", boolean_evaluation.print_formula(&boolean_evaluation.build_tree(&cnf, Some(true)).unwrap()));
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let boolean_evaluation = BooleanOperations::new();
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB|C&"), "AB|C&");
        /*
          The subject writes it as ABC||| but that's just moving the formula from left associative to right associative
          (((A | B) | C) | D) => (A | (B | (C | D)))
          That's not what CNF is about, CNF is about distributing OR over AND
         */
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        /*
            Formula of AB|C&! is (A or B) and (not C)
            The CNF of this formula is C!A!|C!B!|&
            Which is equivalent to (not C or not A) and (not C or not B)
        */
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB|C&!"), "C!A!|C!B!|&");
        assert_eq!(boolean_evaluation.conjunctive_normal_form("ABCD&|&"), "BC|BD|&A&");
        /*
          The subject writes it as ABCD&&& but that's just moving the formula from left associative to right associative
          (((A & B) & C) & D) => (A & (B & (C & D)))
          That's not what CNF is about, CNF is about distributing OR over AND
         */
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB&C&D&"), "AB&C&D&");
        /*
            Formula for AB&!C!| is (A and B) or (not C)
            The CNF of this formula is A!B!|C!|
            Which is equivalent to (not A or not B) or (not C)
            Subject gives us A!B!C!||
            Which is equivalent to (not A) or (not B or not C)
        */
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB&!C!|"), "A!B!|C!|");
        /*
            Formula AB|!C!&  is (A or B) and (not C)
         */
        assert_eq!(boolean_evaluation.conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");

    }
}
