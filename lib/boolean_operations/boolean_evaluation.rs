use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BooleanOperations {
    operations: HashMap<char, fn(bool, bool) -> bool>,
    unary_operations: HashMap<char, fn(bool) -> bool>,
    cache: HashMap<String, bool>,
}
#[derive(Debug, Clone)]
pub enum ExprNode {
    Const(bool),
    Var(char),
    UnaryOp(char, Box<ExprNode>),
    BinaryOp(char, Box<ExprNode>, Box<ExprNode>),
}

impl BooleanOperations {
    pub fn new() -> Self {
        let mut operations: HashMap<char, fn(bool, bool) -> bool> = HashMap::new();
        let mut unary_operations: HashMap<char, fn(bool) -> bool> = HashMap::new();
        let cache: HashMap<String, bool> = HashMap::new();

        operations.insert('>', Self::implication);
        operations.insert('|', Self::disjunction);
        operations.insert('&', Self::conjunction);
        operations.insert('^', Self::exclusive_disjunction);
        operations.insert('=', Self::logical_equivalence);
        unary_operations.insert('!', Self::negation);

        Self {
            operations,
            unary_operations,
            cache,
        }
    }

    /**
    *
    *  Material Conditional (A ⇒ B)
       Truth Table
       A	B	A ⇒ B
       0	0	1
       0	1	1
       1	0	0
       1	1	1
    */
    fn implication(a: bool, b: bool) -> bool {
        Self::disjunction(!a, b)
    }
    fn disjunction(a: bool, b: bool) -> bool {
        a || b
    }

    fn negation(a: bool) -> bool {
        !a
    }

    fn conjunction(a: bool, b: bool) -> bool {
        a && b
    }

    /**
    *
    *  Exclusive Disjunction (A ⊕ B)
       Truth Table
       A	B	A ⊕ B
       0	0	0
       0	1	1
       1	0	1
       1	1	0
    */

    fn exclusive_disjunction(a: bool, b: bool) -> bool {
        a ^ b
    }

    fn logical_equivalence(a: bool, b: bool) -> bool {
        a == b
    }

    /**
     * We can use this function to get a tree representation of the expression
     * If the expression contains variables, we can pass var=true to build the tree
     * with the variables as nodes
     * Otherwise, it'd build the tree with 0 and 1 as nodes, expecting to solve it in the future
     */
    pub fn build_tree(&self, expression: &str, var: Option<bool>) -> Result<ExprNode, String> {
        let mut stack: Vec<ExprNode> = Vec::new();
        let var = var.unwrap_or(false);
        for c in expression.chars() {
            if var && c.is_ascii_uppercase() {
                stack.push(ExprNode::Var(c));
            } else if !var && c == '0' || c == '1' {
                stack.push(ExprNode::Const(c == '1'));
            } else if let Some(_) = self.unary_operations.get(&c) {
                if let Some(expr) = stack.pop() {
                    stack.push(ExprNode::UnaryOp(c, Box::new(expr)));
                } else {
                    return Err(format!(
                        "Error: Not enough operands for unary operator '{}'",
                        c
                    ));
                }
            } else if let Some(_) = self.operations.get(&c) {
                if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
                    stack.push(ExprNode::BinaryOp(c, Box::new(left), Box::new(right)));
                } else {
                    return Err(format!(
                        "Error: Not enough operands for binary operator '{}'",
                        c
                    ));
                }
            } else {
                return Err(format!("Error: Invalid character '{}'", c));
            }
        }

        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        } else {
            Err("Error: Malformed expression".to_string())
        }
    }
    fn evaluate_tree(&mut self, node: &ExprNode) -> bool {
        let key = self.generate_cache_key(node);

        // If result is cached, return it
        if let Some(&cached_result) = self.cache.get(&key) {
            return cached_result;
        }
        let result = match node {
            ExprNode::Const(value) => *value,
            ExprNode::Var(value) => panic!("Can't solve tree with value {}", value),
            ExprNode::UnaryOp(op, expr) => {
                let func: &fn(bool) -> bool = self.unary_operations.get(op).unwrap();
                func(self.evaluate_tree(expr))
            }
            ExprNode::BinaryOp(op, left, right) => {
                let func: &fn(bool, bool) -> bool = self.operations.get(op).unwrap();
                func(self.evaluate_tree(left), self.evaluate_tree(right))
            }
        };
        self.cache.insert(key, result);
        result
    }
    fn generate_cache_key(&self, node: &ExprNode) -> String {
        match node {
            ExprNode::Const(b) => b.to_string(),
            ExprNode::Var(value) => panic!("Can't solve tree with value {}", value),
            ExprNode::UnaryOp(op, expr) => format!("{}{}", op, self.generate_cache_key(expr)),
            ExprNode::BinaryOp(op, left, right) => format!(
                "({} {} {})",
                self.generate_cache_key(left),
                op,
                self.generate_cache_key(right)
            ),
        }
    }

    /**
     * When we evaluate we expect a string with only 0/1 and operators
     */
    pub fn evaluate(&mut self, expression: &str) -> Result<bool, String> {
        let tree: ExprNode = self.build_tree(expression, None)?;
        Ok(self.evaluate_tree(&tree))
    }

    pub(in crate::boolean_operations) fn to_rpn(&self, node: &ExprNode) -> String {
        match node {
            ExprNode::Const(true) => "1".to_string(),
            ExprNode::Const(false) => "0".to_string(),
            ExprNode::Var(c) => c.to_string(),
            ExprNode::UnaryOp(op, child) => {
                let child_rpn = self.to_rpn(child);
                format!("{}{}", child_rpn, op)
            }
            ExprNode::BinaryOp(op, left, right) => {
                let left_rpn = self.to_rpn(left);
                let right_rpn = self.to_rpn(right);
                format!("{}{}{}", left_rpn, right_rpn, op)
            }
        }
    }
    pub fn print_formula(&self, node: &ExprNode) -> String {
        self.print_formula_recursive(node, 0)
    }

    // Helper function for recursive traversal
    fn print_formula_recursive(&self, node: &ExprNode, precedence: u8) -> String {
        match node {
            ExprNode::Const(true) => "1".to_string(),
            ExprNode::Const(false) => "0".to_string(),
            ExprNode::Var(c) => c.to_string(),
            ExprNode::UnaryOp(_op, child) => {
                let child_formula = self.print_formula_recursive(child, 10); 
                format!("¬{}", child_formula)
            }
            ExprNode::BinaryOp(op, left, right) => {
                let (left_formula, right_formula) = match op {
                    '&' => (
                        self.print_formula_recursive(left, 2),
                        self.print_formula_recursive(right, 2),
                    ),
                    '|' => (
                        self.print_formula_recursive(left, 1), 
                        self.print_formula_recursive(right, 1),
                    ),
                    _ => (
                        self.print_formula_recursive(left, 0),
                        self.print_formula_recursive(right, 0),
                    ),
                };
                let formula = match op {
                    '&' => format!("{} ∧ {}", left_formula, right_formula),
                    '|' => format!("{} ∨ {}", left_formula, right_formula),
                    _ => format!("{} {} {}", left_formula, op, right_formula),
                };

                if precedence > 0 {
                    format!("({})", formula)
                } else {
                    formula
                }
            }
        }
    }
}

pub fn run_boolean_operations() {
    println!("\n\tRunning boolean operations function\n");
    let mut boolean_ops = BooleanOperations::new();

    // Test expressions
    let expressions = vec![
        "10&!",
        "11|",
        "10>",
        "01=",
        "10&&",
        "21&",
        "!",
        "",
        "10&00&00&&&",
    ];

    for expr in expressions {
        match boolean_ops.evaluate(expr) {
            Ok(result) => println!("Expression '{}': {}", expr, result),
            Err(e) => println!("Expression '{}': {}", expr, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_implication() {
        assert_eq!(BooleanOperations::implication(false, false), true);
        assert_eq!(BooleanOperations::implication(false, true), true);
        assert_eq!(BooleanOperations::implication(true, false), false);
        assert_eq!(BooleanOperations::implication(true, true), true);
    }
    #[test]
    fn test_disjunction() {
        assert_eq!(BooleanOperations::disjunction(false, false), false);
        assert_eq!(BooleanOperations::disjunction(false, true), true);
        assert_eq!(BooleanOperations::disjunction(true, false), true);
        assert_eq!(BooleanOperations::disjunction(true, true), true);
    }

    #[test]
    fn test_negation() {
        assert_eq!(BooleanOperations::negation(false), true);
        assert_eq!(BooleanOperations::negation(true), false);
    }

    #[test]
    fn test_conjunction() {
        assert_eq!(BooleanOperations::conjunction(false, false), false);
        assert_eq!(BooleanOperations::conjunction(false, true), false);
        assert_eq!(BooleanOperations::conjunction(true, false), false);
        assert_eq!(BooleanOperations::conjunction(true, true), true);
    }

    #[test]
    fn test_exclusive_disjunction() {
        assert_eq!(
            BooleanOperations::exclusive_disjunction(false, false),
            false
        );
        assert_eq!(BooleanOperations::exclusive_disjunction(false, true), true);
        assert_eq!(BooleanOperations::exclusive_disjunction(true, false), true);
        assert_eq!(BooleanOperations::exclusive_disjunction(true, true), false);
    }

    #[test]
    fn test_logical_equivalence() {
        assert_eq!(BooleanOperations::logical_equivalence(false, false), true);
        assert_eq!(BooleanOperations::logical_equivalence(false, true), false);
        assert_eq!(BooleanOperations::logical_equivalence(true, false), false);
        assert_eq!(BooleanOperations::logical_equivalence(true, true), true);
    }
    #[test]
    fn test_evaluate() {
        let mut boolean_operations = BooleanOperations::new();
        assert_eq!(boolean_operations.evaluate("10|").unwrap(), true);
        assert_eq!(boolean_operations.evaluate("10&").unwrap(), false);
        assert_eq!(boolean_operations.evaluate("10>").unwrap(), false);
        assert_eq!(boolean_operations.evaluate("1!").unwrap(), false);
        assert_eq!(boolean_operations.evaluate("11=").unwrap(), true);
        assert_eq!(boolean_operations.evaluate("01^").unwrap(), true);
        assert_eq!(
            boolean_operations
                .evaluate("10&!10&!10&!10&!10&!====")
                .unwrap(),
            true
        );
        assert!(boolean_operations.evaluate("1&").is_err());
        assert!(boolean_operations.evaluate("!").is_err());
        assert!(boolean_operations.evaluate("1@").is_err());
        assert!(boolean_operations.evaluate("").is_err());
    }
}
