use crate::adder::adder;

/*
*   Multiplying two numbers is simply adding the first number to itself the second number of times.
*   This function uses the adder function to multiply two numbers, which in itself uses bitwise operations.
*   We can only use = and ++, but we can do the loop in a more "Rust" way by using 0..b to iterate b times, which is essentially the same as a for loop.
*/
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;
    for _ in 0..b {
        result = adder(result, a);
    }
    return result;
}

pub fn run_multiplier() {
    let a: u32 = 2;
    let b: u32 = 3;
    let product: u32 = multiplier(a, b);

    assert_eq!(product, 6, "Multiplier function did not return the expected result!");

    println!("{} * {} = {}", a, b, product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiplier(2, 3), 6);
        assert_eq!(multiplier(3, 3), 9);
        assert_eq!(multiplier(4, 3), 12);
        assert_eq!(multiplier(0, 3), 0);
    }
}
