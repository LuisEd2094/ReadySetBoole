use crate::aux::traits::Algebra;

pub struct SetOperations;

impl SetOperations {
    pub fn new() -> Self {
        SetOperations
    }
}

impl Algebra<Vec<i32>> for SetOperations {
    // Negation requires the universal set
    fn negation(a: &Vec<i32>, universal: Option<&Vec<Vec<i32>>>) -> Vec<i32> {
        if let Some(universal) = universal {
            if universal.is_empty() || universal[0].is_empty() {
                panic!("Universal set cannot be empty");
            }
            let mut candidates: Vec<i32> = Vec::new();
            for sub_vec in universal {
                for &elem in sub_vec {
                    if !a.contains(&elem) && !candidates.contains(&elem) {
                        candidates.push(elem);
                    }
                }
            }
            candidates
        } else {
            panic!("Universal set is required for negation");
        }
    }

    // Conjunction (AND) - elements in both a AND b
    fn conjunction(a: &Vec<i32>, b: &Vec<i32>, _universal: Option<&Vec<Vec<i32>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for &elem in a {
            if b.contains(&elem) && !result.contains(&elem) {
                result.push(elem);
            }
        }
        result
    }

    // Disjunction (OR) - elements in a OR b
    fn disjunction(a: &Vec<i32>, b: &Vec<i32>, _universal: Option<&Vec<Vec<i32>>>) -> Vec<i32> {
        let mut result: Vec<i32> = a.clone();
        for &elem in b {
            if !result.contains(&elem) {
                result.push(elem);
            }
        }
        result
    }

    // Exclusive disjunction (XOR) - elements in a OR b but not both
    fn exclusive_disjunction(
        a: &Vec<i32>,
        b: &Vec<i32>,
        _universal: Option<&Vec<Vec<i32>>>,
    ) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        // Check A against B
        for &elem in a {
            if !b.contains(&elem) {
                result.push(elem);
            }
        }
        // Check B against A
        for &elem in b {
            if !a.contains(&elem) {
                result.push(elem);
            }
        }
        result
    }

    // Implication (a → b) ≡ ¬a ∨ b
    fn implication(a: &Vec<i32>, b: &Vec<i32>, universal: Option<&Vec<Vec<i32>>>) -> Vec<i32> {
        Self::disjunction(&Self::negation(a, universal), b, universal)
    }

    // Logical equivalence (a ≡ b) ≡ (a → b) ∧ (b → a)
    //                      A↔B≡(A→B)∧(B→A)≡(¬A∪B)∩(¬B∪A)

    fn logical_equivalence(
        a: &Vec<i32>,
        b: &Vec<i32>,
        universal: Option<&Vec<Vec<i32>>>,
    ) -> Vec<i32> {
        let a_imp_b = Self::implication(a, b, universal);
        let b_imp_a = Self::implication(b, a, universal);
        Self::conjunction(&a_imp_b, &b_imp_a, universal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation() {
        let original_universal: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let a = vec![1, 2];

        let result = SetOperations::negation(&a, Some(&original_universal));

        assert_eq!(result, vec![3, 4, 5]);
        assert_eq!(a, vec![1, 2]);
        assert_eq!(original_universal[0], vec![1, 2, 3]);

        ////
        let original_universal: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let a = vec![1, 2, 3, 4, 5];

        let result = SetOperations::negation(&a, Some(&original_universal));

        assert_eq!(result, vec![]);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
        assert_eq!(original_universal[0], vec![1, 2, 3]);
    }

    #[test]
    fn test_conjunction() {
        let a = vec![1, 2];
        let b = vec![3, 4, 5];
        assert_eq!(SetOperations::conjunction(&a, &b, None), vec![]);
        assert_eq!(a, vec![1, 2]);
        assert_eq!(b, vec![3, 4, 5]);

        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(SetOperations::conjunction(&a, &b, None), vec![3]);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![3, 4, 5]);

        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        assert_eq!(SetOperations::conjunction(&a, &b, None), vec![1, 2, 3]);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![1, 2, 3]);

        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        assert_eq!(SetOperations::conjunction(&a, &b, None), vec![]);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![4, 5, 6]);
    }

    #[test]
    fn test_disjunction() {
        let a = vec![1, 2];
        let b = vec![3, 4, 5];
        assert_eq!(
            SetOperations::disjunction(&a, &b, None),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(a, vec![1, 2]);
        assert_eq!(b, vec![3, 4, 5]);

        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(
            SetOperations::disjunction(&a, &b, None),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![3, 4, 5]);

        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        assert_eq!(SetOperations::disjunction(&a, &b, None), vec![1, 2, 3]);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![1, 2, 3]);
    }
    #[test]
    fn test_exclusive_disjunction() {
        let a = vec![1, 2];
        let b = vec![3, 4, 5];
        assert_eq!(
            SetOperations::exclusive_disjunction(&a, &b, None),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(a, vec![1, 2]);
        assert_eq!(b, vec![3, 4, 5]);

        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(
            SetOperations::exclusive_disjunction(&a, &b, None),
            vec![1, 2, 4, 5]
        );
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![3, 4, 5]);

        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        assert_eq!(SetOperations::exclusive_disjunction(&a, &b, None), vec![]);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![1, 2, 3]);
    }

    #[test]
    fn test_implication() {
        let original_universal: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 4, 5]];

        // A ≡ B ⇔ (A → B) ∧ (B → A)
        // A = {3,4}, B = {3,4}
        // A ≡ B = {3,4,5} ∧ {3,4,5} = {3,4,5}
        let a = vec![3, 4];
        let b = vec![3, 4];
        let result = SetOperations::logical_equivalence(&a, &b, Some(&original_universal));
        assert_eq!(result, vec![3, 4, 5]);
        assert_eq!(a, vec![3, 4]);
        assert_eq!(b, vec![3, 4]);

        // A = {1,2}, B = {3,4}
        // A → B = {3,4,5}
        // B → A = {1,2,3}
        // A ≡ B = {3} (only shared element between both)
        let a = vec![1, 2];
        let b = vec![3, 4];
        let result = SetOperations::logical_equivalence(&a, &b, Some(&original_universal));
        assert_eq!(result, vec![3]);

        // Assert nothing was mutated
        assert_eq!(original_universal[0], vec![1, 2, 3]);
        assert_eq!(original_universal[1], vec![3, 4, 5]);
        assert_eq!(a, vec![1, 2]);
        assert_eq!(b, vec![3, 4]);
    }
}

/* use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SetOperations {
    operations: HashMap<char, fn(Vec<i32>, Vec<i32>) -> Vec<i32>>,
    unary_operations: HashMap<char, fn(Vec<i32>) -> Vec<i32>>,
}
#[derive(Debug, Clone)]
pub enum ExprNode {
    Var(Vec<i32>),
    UnaryOp(char, Box<ExprNode>),
    BinaryOp(char, Box<ExprNode>, Box<ExprNode>),
}

impl SetOperations {
    pub fn new() -> Self {
        let mut operations: HashMap<char, fn(Vec<i32>, Vec<i32>) -> Vec<i32>> = HashMap::new();
        let mut unary_operations: HashMap<char, fn(Vec<i32>) -> Vec<i32>> = HashMap::new();

        operations.insert('>', Self::implication);
        operations.insert('|', Self::disjunction);
        operations.insert('&', Self::conjunction);
        operations.insert('^', Self::exclusive_disjunction);
        operations.insert('=', Self::logical_equivalence);
        unary_operations.insert('!', Self::negation);


        SetOperations { operations, unary_operations }
    }


}

fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    // Check if number of sets matches variables in formula
    let num_vars = formula.chars().filter(|c| c.is_ascii_uppercase()).count();
    if num_vars != sets.len() {
        eprintln!("Error: Expected {} sets, got {}", num_vars, sets.len());
        return Vec::new();
    }

    // Convert sets to HashSets for easier operations
    let sets: Vec<HashSet<i32>> = sets.into_iter().map(|v| v.into_iter().collect()).collect();

    // Get universal set (union of all sets)
    let universal: HashSet<i32> = sets.iter().flatten().copied().collect();

    let mut stack: Vec<HashSet<i32>> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                let idx = (c as u8 - b'A') as usize;
                if idx >= sets.len() {
                    eprintln!("Error: Undefined set {}", c);
                    return Vec::new();
                }
                stack.push(sets[idx].clone());
            },
            '!' => {
                if let Some(a) = stack.pop() {
                    let complement = universal.difference(&a).copied().collect();
                    stack.push(complement);
                } else {
                    eprintln!("Error: Not enough operands for !");
                    return Vec::new();
                }
            },
            '&' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let intersection = a.intersection(&b).copied().collect();
                    stack.push(intersection);
                } else {
                    eprintln!("Error: Not enough operands for &");
                    return Vec::new();
                }
            },
            '|' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let union = a.union(&b).copied().collect();
                    stack.push(union);
                } else {
                    eprintln!("Error: Not enough operands for |");
                    return Vec::new();
                }
            },
            '^' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let sym_diff = a.symmetric_difference(&b).copied().collect();
                    stack.push(sym_diff);
                } else {
                    eprintln!("Error: Not enough operands for ^");
                    return Vec::new();
                }
            },
            '>' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let implication = universal.difference(&a).union(&b).copied().collect();
                    stack.push(implication);
                } else {
                    eprintln!("Error: Not enough operands for >");
                    return Vec::new();
                }
            },
            '=' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let eq = a.symmetric_difference(&b).collect::<HashSet<_>>();
                    let result = universal.difference(&eq).copied().collect();
                    stack.push(result);
                } else {
                    eprintln!("Error: Not enough operands for =");
                    return Vec::new();
                }
            },
            _ => {
                eprintln!("Error: Invalid character '{}' in formula", c);
                return Vec::new();
            }
        }
    }

    if stack.len() != 1 {
        eprintln!("Error: Invalid formula - stack has {} elements at end", stack.len());
        return Vec::new();
    }

    let mut result: Vec<i32> = stack.pop().unwrap().into_iter().collect();
    result.sort(); // For consistent output in tests
    result
}

fn main() {
    // Test cases
    let sets1 = vec![vec![0, 1, 2], vec![0, 3, 4]];
    assert_eq!(eval_set("AB&", sets1), vec![0]);

    let sets2 = vec![vec![0, 1, 2], vec![3, 4, 5]];
    assert_eq!(eval_set("AB|", sets2), vec![0, 1, 2, 3, 4, 5]);

    let sets3 = vec![vec![0, 1, 2]];
    assert_eq!(eval_set("A!", sets3), vec![]);

    println!("All tests passed!");
} */
