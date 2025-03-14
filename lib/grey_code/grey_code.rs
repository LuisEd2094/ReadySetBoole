/**
*
   Decimal	Binary	Gray
   0	0000	0000
   1	0001	0001
   2	0010	0011
   3	0011	0010
   4	0100	0110
   5	0101	0111
   6	0110	0101
   7	0111	0100
   8	1000	1100
   9	1001	1101
   10	1010	1111
   11	1011	1110
   12	1100	1010
   13	1101	1011
   14	1110	1001
   15	1111	1000
*/
use crate::aux::dec_to_bin::to_binary;

/**
 * Grey code is a binary numeral system where two successive values differ in only one bit.
 * Moving N one step to the right keeps the MSB the same, the XOR ensures only one bit is changed.
 */
pub fn grey_code(n: u32) -> u32 {
    return n ^ (n >> 1);
}

pub fn run_grey_code() {
    println!("\nRunning grey code function\n");
    let n: u32 = 15;
    let grey: u32 = grey_code(n);

    println!("Grey code of {} is {}", n, grey);
    println!(
        "Binaries. Current: {} Prev: {} Next: {}",
        to_binary(grey),
        to_binary(grey_code(n - 1)),
        to_binary(grey_code(n + 1))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grey_code() {
        assert_eq!(grey_code(1), 1);
        assert_eq!(grey_code(2), 3);
        assert_eq!(grey_code(3), 2);
        assert_eq!(grey_code(4), 6);
        assert_eq!(grey_code(0), 0);
        assert_eq!(grey_code(15), 8);
        assert_eq!(grey_code(14), 9);
        assert_eq!(grey_code(16), 24);
    }
}
