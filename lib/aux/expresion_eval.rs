use crate::aux::Algebra;
use regex::Regex;
use std::collections::HashMap;

pub struct ExpressionEvaluator<T, O: Algebra<T>> {
    operations: HashMap<char, fn(&T, &T, Option<&Vec<T>>) -> T>,
    unary_operations: HashMap<char, fn(&T, Option<&Vec<T>>) -> T>,
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
            _ => panic!("Cannot convert set to bool"),
        }
    }
}
impl From<LogicValue> for Vec<i32> {
    fn from(value: LogicValue) -> Vec<i32> {
        match value.value {
            ValueType::Set(s) => s,
            _ => panic!("Cannot convert bool to set"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValueType {
    Bool(bool),
    Set(Vec<i32>),
}

#[derive(Debug, Clone)]
pub enum ExprNode<T> {
    Const(T),
    Var(char),
    UnaryOp(char, Box<ExprNode<T>>),
    BinaryOp(char, Box<ExprNode<T>>, Box<ExprNode<T>>),
}

impl<T, O> ExpressionEvaluator<T, O>
where
    T: From<LogicValue> + Clone + std::fmt::Debug, // T must be Debug
    O: Algebra<T>,
{
    pub fn new() -> Self {
        let mut operations: HashMap<char, fn(&T, &T, Option<&Vec<T>>) -> T> = HashMap::new();
        let mut unary_operations: HashMap<char, fn(&T, Option<&Vec<T>>) -> T> = HashMap::new();

        operations.insert('>', O::implication as fn(&T, &T, Option<&Vec<T>>) -> T);
        operations.insert('|', O::disjunction as fn(&T, &T, Option<&Vec<T>>) -> T);
        operations.insert('&', O::conjunction as fn(&T, &T, Option<&Vec<T>>) -> T);
        operations.insert(
            '^',
            O::exclusive_disjunction as fn(&T, &T, Option<&Vec<T>>) -> T,
        );
        operations.insert(
            '=',
            O::logical_equivalence as fn(&T, &T, Option<&Vec<T>>) -> T,
        );

        unary_operations.insert('!', O::negation as fn(&T, Option<&Vec<T>>) -> T);

        Self {
            operations,
            unary_operations,
            cache: HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    fn validate_hash(
        &self,
        expression: &str,
        hash: Option<&HashMap<String, T>>,
    ) -> Result<bool, String> {
        match hash {
            Some(h) => {
                for key in h.keys() {
                    if !expression.contains(key) {
                        return Err(format!("Variable '{}' not found", key));
                    }
                }
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub fn build_tree(
        &self,
        expression: &str,
        var: bool,
        hash: Option<&HashMap<String, T>>,
    ) -> Result<ExprNode<T>, String> {
        let use_hash = self.validate_hash(expression, hash)?;
        let mut stack: Vec<ExprNode<T>> = Vec::new();
        for c in expression.chars() {
            if c.is_ascii_uppercase() {
                if use_hash {
                    if let Some(hash) = hash {
                        if let Some(value) = hash.get(&c.to_string()) {
                            stack.push(ExprNode::Const(value.clone()));
                        }
                    }
                } else if var {
                    stack.push(ExprNode::Var(c));
                } else {
                    return Err(format!("Error: Invalid character '{}'", c));
                }
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

    fn evaluate_tree(&mut self, node: &ExprNode<T>, universal: Option<&Vec<T>>) -> T {
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
                func(&self.evaluate_tree(expr, universal), universal)
            }
            ExprNode::BinaryOp(op, left, right) => {
                let func = self.operations.get(op).unwrap();
                func(
                    &self.evaluate_tree(left, universal),
                    &self.evaluate_tree(right, universal),
                    universal,
                )
            }
        };
        self.cache.insert(key, result.clone());
        result
    }

    fn generate_cache_key(&self, node: &ExprNode<T>) -> String {
        match node {
            ExprNode::Const(value) => format!("{:?}", value),
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

    pub fn evaluate(
        &mut self,
        expression: &str,
        variables_value: Option<&HashMap<String, T>>,
    ) -> Result<T, String> {
        let binary_re = Regex::new(r"^[A-Z!&|ˆ>=]+$").unwrap().is_match(expression);
        let tree = self.build_tree(expression, binary_re, variables_value)?;

        let universal_values: Option<Vec<T>> =
            variables_value.map(|map| map.values().cloned().collect());
        Ok(self.evaluate_tree(&tree, universal_values.as_ref()))
    }

    pub fn to_rpn(&self, node: &ExprNode<T>) -> String {
        match node {
            ExprNode::Const(value) => format!("{:?}", value),
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

    pub fn print_formula(&self, node: &ExprNode<T>) -> String {
        self.print_formula_recursive(node, 0)
    }

    fn print_formula_recursive(&self, node: &ExprNode<T>, precedence: u8) -> String {
        match node {
            ExprNode::Const(value) => format!("{:?}", value),
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