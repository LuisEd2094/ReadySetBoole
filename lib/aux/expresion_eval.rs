use crate::aux::Algebra;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct ExpressionEvaluator<T, O: Algebra<T>> {
    operations: HashMap<char, fn(T, T) -> T>,
    unary_operations: HashMap<char, fn(T) -> T>,
    cache: HashMap<String, T>,
    _marker: std::marker::PhantomData<O>,
}

#[derive(Debug, Clone)]
pub struct LogicValue {
    pub value: ValueType,
}

impl From<LogicValue> for bool {
    fn from(value: LogicValue) -> bool {
        match value.value {
            ValueType::Bool(b) => b,
            ValueType::Set(_) => panic!("Cannot convert set to bool"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValueType {
    Bool(bool),
    Set(HashSet<bool>),
}

#[derive(Debug, Clone)]
pub enum ExprNode<T> {
    Const(T),
    Var(char),
    UnaryOp(char, Box<ExprNode<T>>),
    BinaryOp(char, Box<ExprNode<T>>, Box<ExprNode<T>>),
}

impl<T, O: Algebra<T>> ExpressionEvaluator<T, O> {
    pub fn new() -> Self {
        let mut operations: HashMap<char, fn(T, T) -> T> =
            HashMap::new();
        let mut unary_operations: HashMap<char, fn(T) -> T> = HashMap::new();

        operations.insert(
            '>',
            O::implication as fn(T, T) -> T,
        );
        operations.insert(
            '|',
            O::disjunction as fn(T, T) -> T,
        );
        operations.insert(
            '&',
            O::conjunction as fn(T, T) -> T,
        );
        operations.insert(
            '^',
            O::exclusive_disjunction as fn(T, T) -> T,
        );
        operations.insert(
            '=',
            O::logical_equivalence as fn(T, T) -> T,
        );

        unary_operations.insert('!', O::negation as fn(T) -> T);

        Self {
            operations,
            unary_operations,
            cache: HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build_tree(&self, expression: &str, var: bool) -> Result<ExprNode<T>, String>
    where
        T: From<LogicValue>,
    {
        let mut stack: Vec<ExprNode<T>> = Vec::new();
        for c in expression.chars() {
            if var && c.is_ascii_uppercase() {
                stack.push(ExprNode::Var(c));
            } else if !var && (c == '0' || c == '1') {
                stack.push(ExprNode::Const(
                    LogicValue {
                        value: ValueType::Bool(c == '1'),
                    }
                    .into(),
                ));
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

    fn evaluate_tree(&mut self, node: &ExprNode<T>) -> T
    where
        T: Clone + std::fmt::Display,
    {
        let key = self.generate_cache_key(node);

        // If result is cached, return it
        if let Some(cached_result) = self.cache.get(&key) {
            return cached_result.clone();
        }
        let result = match node {
            ExprNode::Const(value) => value.clone(),
            ExprNode::Var(value) => panic!("Can't solve tree with value {}", value),
            ExprNode::UnaryOp(op, expr) => {
                let func = self.unary_operations.get(op).unwrap();
                func(self.evaluate_tree(expr))
            }
            ExprNode::BinaryOp(op, left, right) => {
                let func = self.operations.get(op).unwrap();
                func(self.evaluate_tree(left), self.evaluate_tree(right))
            }
        };
        self.cache.insert(key, result.clone());
        result
    }

    fn generate_cache_key(&self, node: &ExprNode<T>) -> String
    where
        T: std::fmt::Display,
    {
        match node {
            ExprNode::Const(value) => value.to_string(),
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

    pub fn evaluate(&mut self, expression: &str) -> Result<T, String>
    where
        T: std::fmt::Display + Clone + From<LogicValue>,
    {
        let binary_re = Regex::new(r"^[A-Z!&|ˆ>=]+$").unwrap().is_match(expression);
        let tree = self.build_tree(expression, binary_re)?;
        Ok(self.evaluate_tree(&tree))
    }

    pub fn to_rpn(&self, node: &ExprNode<T>) -> String
    where
        T: std::fmt::Display + Clone + From<LogicValue>,
    {
        match node {
            ExprNode::Const(value) => value.to_string(),
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

    pub fn print_formula(&self, node: &ExprNode<T>) -> String
    where
        T: std::fmt::Display,
    {
        self.print_formula_recursive(node, 0)
    }

    fn print_formula_recursive(&self, node: &ExprNode<T>, precedence: u8) -> String
    where
        T: std::fmt::Display,
    {
        match node {
            ExprNode::Const(value) => value.to_string(),
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

/* impl BooleanOperations {
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
} */
