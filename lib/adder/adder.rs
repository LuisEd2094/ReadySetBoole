pub fn adder(a: u32, b: u32) -> u32 {
    let mut x: u32 = a;
    let mut y: u32 = b;
    while y != 0 {
        let carry: u32 = x & y; // Carry now contains common set bits of a and b
        x = x ^ y; // Sum of bits of a and b where at least one of the bits is not set
        y = carry << 1; // Carry is shifted by one so that adding it to a gives the required sum
    }
    return x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(adder(2, 3), 5);
    }
}
