use crate::boolean_evaluation::{BooleanOperations, ExprNode};

pub fn negation_normal_form(formula: &str) -> String {
    let operations = BooleanOperations::new();

    let tree = operations
        .build_tree(formula, Some(true))
        .expect("Failed to build tree");

    // Convert the tree to NNF
    let nnf_tree = to_nnf(tree);

    // Convert the NNF tree back to RPN
    to_rpn(&nnf_tree)
}

fn to_nnf(node: ExprNode) -> ExprNode {
    match node {
        // Handle double negation
        ExprNode::UnaryOp('!', child) => match *child {
            ExprNode::UnaryOp('!', child2) => to_nnf(*child2), // !!A => A
            ExprNode::BinaryOp(op, left, right) => {
                // De Morgan's laws: !(A & B) => !A | !B, !(A | B) => !A & !B
                let new_op = match op {
                    '&' => '|',
                    '|' => '&',
                    _ => op,
                };
                ExprNode::BinaryOp(
                    new_op,
                    Box::new(to_nnf(ExprNode::UnaryOp('!', left))),
                    Box::new(to_nnf(ExprNode::UnaryOp('!', right))),
                )
            }
            _ => ExprNode::UnaryOp('!', Box::new(to_nnf(*child))),
        },
        // Handle binary operators
        ExprNode::BinaryOp(op, left, right) => {
            let left_nnf = to_nnf(*left);
            let right_nnf = to_nnf(*right);
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
                    let and_expr = ExprNode::BinaryOp('&', Box::new(left_nnf), Box::new(right_nnf));
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

fn to_rpn(node: &ExprNode) -> String {
    match node {
        ExprNode::Const(true) => "1".to_string(),
        ExprNode::Const(false) => "0".to_string(),
        ExprNode::Var(c) => c.to_string(),
        ExprNode::UnaryOp(op, child) => {
            let child_rpn = to_rpn(child);
            format!("{}{}", child_rpn, op)
        }
        ExprNode::BinaryOp(op, left, right) => {
            let left_rpn = to_rpn(left);
            let right_rpn = to_rpn(right);
            format!("{}{}{}", left_rpn, right_rpn, op)
        }
    }
}

pub fn run_negation_normal_form() {
    println!("\n\tRunning negation normal form function\n");
    let formula = "AB&!";
    println!("Original formula: {}", formula);
    let nnf = negation_normal_form(formula);
    println!("Negation Normal Form: {}", nnf);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "A!B|B!A|&");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }
}
