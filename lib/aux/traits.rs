pub trait Algebra<T> {
    fn implication(a: T, b: T) -> T;
    fn disjunction(a: T, b: T) -> T;
    fn negation(a: T) -> T;
    fn conjunction(a: T, b: T) -> T;
    fn exclusive_disjunction(a: T, b: T) -> T;
    fn logical_equivalence(a: T, b: T) -> T;
}

pub trait SatTruthTable {
    fn sat_truth_table(&mut self, formula: &str) -> bool;
}
