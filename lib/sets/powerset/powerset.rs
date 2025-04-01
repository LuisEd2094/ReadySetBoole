pub fn powerset(set: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut subsets: Vec<Vec<i32>> = vec![vec![]];
    for &num in set {
        let mut new_subsets: Vec<Vec<i32>> = Vec::new();
        for subset in &subsets {
            let mut new_subset: Vec<i32> = subset.clone();
            new_subset.push(num);
            new_subsets.push(new_subset);
        }
        subsets.extend(new_subsets);
    }
    subsets
}

pub fn run_powerset(){
    println!("\t\nRunning powerset...\n");
    let set: Vec<i32> = vec![1, 2, 3];
    let result: Vec<Vec<i32>> = powerset(&set);
    println!("Powerset of {:?} is: {:?}", set, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerset() {
        let set = vec![1, 2, 3];
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let mut result = powerset(&set);
        assert_eq!(result.sort(), expected.sort());

        let set = vec![1];
        let mut expected: Vec<Vec<i32>> = vec![vec![], vec![1]];
        let mut result: Vec<Vec<i32>> = powerset(&set);
        assert_eq!(result.sort(), expected.sort());


        let set = vec![1,2,3,4];
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        let mut result: Vec<Vec<i32>> = powerset(&set);
        assert_eq!(result.sort(), expected.sort());

    }
}
