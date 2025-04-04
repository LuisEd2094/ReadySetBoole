use crate::aux::check_only_vars;
use crate::aux::expresion_eval::LogicValue;
use crate::aux::expresion_eval::{ExprNode, ExpressionEvaluator};
use crate::aux::traits::Algebra;
use crate::boolean_operations::BooleanOperations;

impl<T, O: Algebra<T>> ExpressionEvaluator<T, O>
where
    T: From<LogicValue> + std::fmt::Display + Clone,
{
    pub fn negation_normal_form(&self, formula: &str) -> String {
        let tree = self
            .build_tree(formula, check_only_vars(formula))
            .expect("Failed to build tree");

        // Convert the tree to NNF
        let nnf_tree = self.to_nnf(tree);

        // Convert the NNF tree back to RPN
        self.to_rpn(&nnf_tree)
    }

    pub(in crate::boolean_operations) fn to_nnf(
        &self,
        node: ExprNode<T>,
    ) -> ExprNode<T>
    where
        T: From<LogicValue> + std::fmt::Display + Clone,
    {
        match node {
            // Handle double negation
            ExprNode::UnaryOp('!', child) => match *child {
                ExprNode::UnaryOp('!', child2) => self.to_nnf(*child2), // !!A => A
                ExprNode::BinaryOp(op, left, right) => {
                    // De Morgan's laws: !(A & B) => !A | !B, !(A | B) => !A & !B
                    let new_op = match op {
                        '&' => '|',
                        '|' => '&',
                        _ => op,
                    };
                    ExprNode::BinaryOp(
                        new_op,
                        Box::new(self.to_nnf(ExprNode::UnaryOp('!', left))),
                        Box::new(self.to_nnf(ExprNode::UnaryOp('!', right))),
                    )
                }
                _ => ExprNode::UnaryOp('!', Box::new(self.to_nnf(*child))),
            },
            // Handle binary operators
            ExprNode::BinaryOp(op, left, right) => {
                let left_nnf = self.to_nnf(*left);
                let right_nnf = self.to_nnf(*right);
                match op {
                    '&' => ExprNode::BinaryOp('&', Box::new(left_nnf), Box::new(right_nnf)),
                    '|' => ExprNode::BinaryOp('|', Box::new(left_nnf), Box::new(right_nnf)),
                    'ˆ' => {
                        // A ⊕ B => (A | B) & !(A & B)
                        let or_expr = ExprNode::BinaryOp(
                            '|',
                            Box::new(left_nnf.clone()),
                            Box::new(right_nnf.clone()),
                        );
                        let and_expr =
                            ExprNode::BinaryOp('&', Box::new(left_nnf), Box::new(right_nnf));
                        ExprNode::BinaryOp(
                            '&',
                            Box::new(or_expr),
                            Box::new(ExprNode::UnaryOp('!', Box::new(and_expr))),
                        )
                    }
                    '>' => {
                        // A ⇒ B => !A | B
                        let not_left = ExprNode::UnaryOp('!', Box::new(left_nnf));
                        ExprNode::BinaryOp('|', Box::new(not_left), Box::new(right_nnf))
                    }
                    '=' => {
                        // A ⇔ B => (A ⇒ B) & (B ⇒ A)
                        let left_to_right = ExprNode::BinaryOp(
                            '|',
                            Box::new(ExprNode::UnaryOp('!', Box::new(left_nnf.clone()))),
                            Box::new(right_nnf.clone()),
                        );
                        let right_to_left = ExprNode::BinaryOp(
                            '|',
                            Box::new(ExprNode::UnaryOp('!', Box::new(right_nnf))),
                            Box::new(left_nnf),
                        );
                        ExprNode::BinaryOp('&', Box::new(left_to_right), Box::new(right_to_left))
                    }
                    _ => ExprNode::BinaryOp(op, Box::new(left_nnf), Box::new(right_nnf)),
                }
            }
            // Handle constants and variables
            _ => node,
        }
    }
}

pub fn run_negation_normal_form() {
    let boolean_evaluation: ExpressionEvaluator<bool, BooleanOperations> =
        ExpressionEvaluator::<bool, BooleanOperations>::new();
    println!("\n\tRunning negation normal form function\n");
    let formula = "AB&!";
    println!("Original formula: {}", formula);
    let nnf = boolean_evaluation.negation_normal_form(formula);
    println!("Negation Normal Form: {}", nnf);
    println!(
        "Formula {}",
        boolean_evaluation.print_formula(
            &boolean_evaluation
                .build_tree(&nnf, check_only_vars(&nnf))
                .unwrap()
        )
    );
}

#[cfg(test)]
mod tests {
    use crate::truth_table::generate_truth_table;

    use super::*;
    #[test]
    fn test_transformation() {
        let boolean_evaluation: ExpressionEvaluator<bool, BooleanOperations> =
            ExpressionEvaluator::<bool, BooleanOperations>::new();
        assert_eq!(boolean_evaluation.negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(boolean_evaluation.negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(boolean_evaluation.negation_normal_form("AB>"), "A!B|");
        assert_eq!(boolean_evaluation.negation_normal_form("AB="), "A!B|B!A|&");
        assert_eq!(
            boolean_evaluation.negation_normal_form("AB|C&!"),
            "A!B!&C!|"
        );

        // No transformation needed
        assert_eq!(
            boolean_evaluation.negation_normal_form("A!B!&C!|"),
            "A!B!&C!|"
        );
        assert_eq!(boolean_evaluation.negation_normal_form("A!B!|"), "A!B!|");
        assert_eq!(boolean_evaluation.negation_normal_form("A!B!&"), "A!B!&");
        assert_eq!(boolean_evaluation.negation_normal_form("A!B|"), "A!B|");
        assert_eq!(
            boolean_evaluation.negation_normal_form("A!B|B!A|&"),
            "A!B|B!A|&"
        );
    }
    #[test]
    fn test_truth_table() {
        let mut evaluator: ExpressionEvaluator<bool, BooleanOperations> =
            ExpressionEvaluator::<bool, BooleanOperations>::new();
        let formula = "AB&!";
        let nnf = evaluator.negation_normal_form(formula);
        assert_eq!(
            generate_truth_table(formula, &mut evaluator).unwrap(),
            generate_truth_table(&nnf, &mut evaluator).unwrap()
        );
    }
}
