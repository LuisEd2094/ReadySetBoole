use std::collections::HashMap;

pub struct BooleanOperations {
    operations: HashMap<char, fn(bool, bool) -> bool>,
    unary_operations: HashMap<char, fn(bool) -> bool>,
}

impl BooleanOperations {
    pub fn new() -> Self {
        let mut operations: HashMap<char, fn(bool, bool) -> bool> = HashMap::new();
        let mut unary_operations: HashMap<char, fn(bool) -> bool> = HashMap::new();

        operations.insert('>', Self::implication);
        operations.insert('|', Self::disjunction);
        operations.insert('&', Self::conjunction);
        operations.insert('^', Self::exclusive_disjunction);
        operations.insert('=', Self::logical_equivalence);
        unary_operations.insert('!', Self::negation);

        Self {
            operations,
            unary_operations,
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
    pub fn evaluate(&mut self, expression: &str) -> Result<bool, String> {
        let mut stack: Vec<bool> = vec![];

        for c in expression.chars() {
            if self.unary_operations.contains_key(&c) {
                if let Some(a) = stack.pop() {
                    let result = self.unary_operations.get(&c).unwrap()(a);
                    stack.push(result);
                } else {
                    return Err(format!(
                        "Error: Not enough operands for unary operator '{}'", c
                    ));
                }
            } else if self.operations.contains_key(&c) {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let result = self.operations.get(&c).unwrap()(a, b);
                    stack.push(result);
                } else {
                    return Err(format!(
                        "Error: Not enough operands for binary operator '{}'", c
                    ));
                }
            } else if c == '0' || c == '1' {
                stack.push(c == '1');
            } else {
                return Err(format!("Error: Invalid character '{}'", c));
            }
        }

        // Final result
        if let Some(result) = stack.pop() {
            if stack.is_empty() {
                Ok(result)
            } else {
                Err("Error: Extra operands left in stack".to_string())
            }
        } else {
            Err("Error: Empty expression".to_string())
        }
    }
}

pub fn run_boolean_operations() {
    println!("\nRunning boolean operations function\n");
    let mut boolean_ops = BooleanOperations::new();

    // Test expressions
    let expressions = vec!["10&!", "11|", "10>", "01=", "10&&", "21&", "!", ""];

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

        assert!(boolean_operations.evaluate("1&").is_err());
        assert!(boolean_operations.evaluate("!").is_err());
        assert!(boolean_operations.evaluate("1@").is_err());
        assert!(boolean_operations.evaluate("").is_err());
    }
}
