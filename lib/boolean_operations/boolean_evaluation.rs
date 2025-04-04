use crate::aux::expresion_eval::ExpressionEvaluator;
use crate::aux::traits::Algebra;

#[derive(Debug, Clone)]
pub struct BooleanOperations;

impl BooleanOperations {
    pub fn new() -> Self {
        BooleanOperations
    }
}

impl Algebra<bool> for BooleanOperations {
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
}

pub fn run_boolean_operations() {
    println!("\n\tRunning boolean operations function\n");
    let mut boolean_ops: ExpressionEvaluator<bool, BooleanOperations> =
        ExpressionEvaluator::<bool, BooleanOperations>::new();

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
        let mut boolean_operations: ExpressionEvaluator<bool, BooleanOperations> =
            ExpressionEvaluator::<bool, BooleanOperations>::new();
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
