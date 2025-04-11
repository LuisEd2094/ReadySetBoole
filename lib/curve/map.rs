pub fn map(x: u16, y: u16) -> f64 {
    fn part1by1(n: u16) -> u32 {
        let mut n: u32 = n as u32;
        n = (n | (n << 8)) & 0x00FF00FF;
        n = (n | (n << 4)) & 0x0F0F0F0F;
        n = (n | (n << 2)) & 0x33333333;
        n = (n | (n << 1)) & 0x55555555;
        n
    }

    let interleaved: u32 = (part1by1(y) << 1) | part1by1(x);
    let max_val: f64 = u32::MAX as f64;

    // normalize to [0, 1]
    interleaved as f64 / max_val
}

pub fn run_map()
{
    println!("\n\tRunning map function\n");
    let (x, y) = (12345, 6789);
    let val: f64 = map(x, y);

    println!("The map x: {}, y: {} is {} ", val, x, y);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_range_bounds() {
        let vals: Vec<f64> = vec![
            map(0, 0),
            map(65535, 0),
            map(0, 65535),
            map(65535, 65535),
            map(32768, 32768),
            map(1, 0)
        ];
        for v in vals {
            assert!(v >= 0.0 && v <= 1.0, "Value {} is out of range [0,1]", v);
        }
    }
    #[test]
    fn test_map_uniqueness() {
        let a: f64 = map(12345, 6789);
        let b: f64 = map(12345, 6790);
        let c: f64 = map(12346, 6789);
        assert!(a != b, "map(x, y) should be unique: a == b");
        assert!(a != c, "map(x, y) should be unique: a == c");
        assert!(b != c, "map(x, y) should be unique: b == c");
    }

    #[test]
    fn test_map_order_behavior() {
        let low: f64 = map(0, 0);
        let mid: f64 = map(32767, 32767);
        let high: f64 = map(65535, 65535);
        assert!(low < mid, "Expected low < mid");
        assert!(mid < high, "Expected mid < high");
    }
    #[test]
    fn test_map_consistency() {
        let a = map(10000, 20000);
        let b = map(10000, 20000);
        assert_eq!(a, b, "Same input should yield same output");
    }

    #[test]
    fn test_map_known_values() {
        let corner = map(0, 0);
        assert_eq!(corner, 0.0);

        let max = map(65535, 65535);
        assert!((max - 1.0).abs() < 1e-9, "Expected max value to be ~1.0");
    }
}
