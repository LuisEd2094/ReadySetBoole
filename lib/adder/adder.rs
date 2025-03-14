/*
 *  Binary sums work similarly to decimal sums, except we carry over the one when we go to "2".
 * 0 + 0 = 0
 * 0 + 1 = 1
 * 1 + 0 = 1
 * 1 + 1 = 0, carry 1
 *
 * As we can see, we only carry the one when both bits are 1, a AND operation will give us the bits that we need to carry over.
 * an XOR operation will give us the bits that we need to sum, since it'd gives us 1 when the bits are different. (0 + 1 = 1, 1 + 0 = 1)
 * and it'd give us 0 when the bits are the same. (0 + 0 = 0, 1 + 1 = 0)
 *
 * Lastly, we shift the carry over to the left by 1, since we're carrying over to the next bit.
 *
 * AND truth table:
 * 0 0 = 0
 * 0 1 = 0
 * 1 0 = 0
 * 1 1 = 1
 *
 * XOR truth table:
 * 0 0 = 0
 * 0 1 = 1
 * 1 0 = 1
 * 1 1 = 0
 */
pub fn adder(a: u32, b: u32) -> u32 {
    let mut x: u32 = a;
    let mut y: u32 = b;
    let mut carry: u32 = 1;
    while carry != 0 {
        carry = x & y;
        x = x ^ y;
        y = carry << 1;
    }
    return x;
}

pub fn run_adder() {
    let a: u32 = 2;
    let b: u32 = 3;
    let sum: u32 = adder(a, b);

    assert_eq!(sum, 5, "Adder function did not return the expected result!");

    println!("{} + {} = {}", a, b, sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(adder(2, 3), 5);
        assert_eq!(adder(3, 3), 6);
        assert_eq!(adder(4, 3), 7);
        assert_eq!(adder(0, 3), 3);
        assert_eq!(adder(0, 0), 0);
    }
}
