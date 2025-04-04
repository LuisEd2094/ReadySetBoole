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
