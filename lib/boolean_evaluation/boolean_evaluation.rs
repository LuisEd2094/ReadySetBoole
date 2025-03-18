use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ExprNode {
    Const(bool),
    UnaryOp(char, Box<ExprNode>),
    BinaryOp(char, Box<ExprNode>, Box<ExprNode>),
}
pub struct BooleanOperations {
    operations: HashMap<char, fn(bool, bool) -> bool>,
    unary_operations: HashMap<char, fn(bool) -> bool>,
    cache: HashMap<String, bool>,
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

    fn build_tree(&self, expression: &str) -> Result<ExprNode, String> {
        let mut stack: Vec<ExprNode> = Vec::new();

        for c in expression.chars() {
            if c == '0' || c == '1' {
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
            ExprNode::UnaryOp(op, expr) => format!("{}{}", op, self.generate_cache_key(expr)),
            ExprNode::BinaryOp(op, left, right) => format!(
                "({} {} {})",
                self.generate_cache_key(left),
                op,
                self.generate_cache_key(right)
            ),
        }
    }
    pub fn evaluate(&mut self, expression: &str) -> Result<bool, String> {
        let tree: ExprNode = self.build_tree(expression)?;
        Ok(self.evaluate_tree(&tree))
    }
}

pub fn run_boolean_operations() {
    println!("\nRunning boolean operations function\n");
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
