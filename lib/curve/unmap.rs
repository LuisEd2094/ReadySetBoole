pub fn reverse_map(value: f64) -> (u16, u16) {
    fn compact1by1(n: u32) -> u16 {
        let mut n = n & 0x55555555;
        n = (n | (n >> 1)) & 0x33333333;
        n = (n | (n >> 2)) & 0x0F0F0F0F;
        n = (n | (n >> 4)) & 0x00FF00FF;
        n = (n | (n >> 8)) & 0x0000FFFF;
        n as u16
    }

    let max_val = u32::MAX as f64;
    let interleaved = (value * max_val).round() as u32;

    let x = compact1by1(interleaved);
    let y = compact1by1(interleaved >> 1);

    (x, y)
}

pub fn run_reverse_map()
{
    println!("\n\tRunning reverse_map function\n");
    let val: f64 = 1.0;
    let (x, y) = reverse_map(val);
    println!("The reverse of {} is x: {}, y: {}", val, x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_map_zero() {
        // interleaved = 0 → (x, y) = (0, 0)
        let val: f64 = 0.0;
        assert_eq!(reverse_map(val), (0, 0));
    }

    #[test]
    fn test_reverse_map_one() {
        // interleaved = 1 → x = 1, y = 0 (because x is in LSBs)
        let val: f64 = 1.0 / u32::MAX as f64;
        assert_eq!(reverse_map(val), (1, 0));
    }

    #[test]
    fn test_reverse_map_two() {
        let val: f64 = 2.0 / u32::MAX as f64;
        assert_eq!(reverse_map(val), (0, 1));
    }

    #[test]
    fn test_reverse_map_four() {
        let val: f64 = 4.0 / u32::MAX as f64;
        let result: (u16, u16) = reverse_map(val);
        assert_eq!(result, (2, 0));
    }

    #[test]
    fn test_reverse_map_max() {
        let val: f64 = 1.0;
        assert_eq!(reverse_map(val), (65535, 65535));
    }

    #[test]
    fn test_map_reverse_map_bijection() {
        use crate::curve::map::map;
        let test_cases: Vec<(u16, u16)> = vec![
            (0, 0),
            (1, 0),
            (0, 1),
            (1, 1),
            (65535, 0),
            (0, 65535),
            (65535, 65535),
            (12345, 6789),
            (32768, 32768),
        ];

        for (x, y) in test_cases {
            let mapped: f64 = map(x, y);
            let (reverse_mapped_x, reverse_mapped_y) = reverse_map(mapped);
            assert_eq!(
                (x, y),
                (reverse_mapped_x, reverse_mapped_y),
                "Failed for input ({}, {}): mapped to {}, reverse mapped to ({}, {})",
                x,
                y,
                mapped,
                reverse_mapped_x,
                reverse_mapped_y
            );
        }
    }
}
