/// Round a float to a given number of digits.
pub fn round_to_digits(x: f64, n: usize) -> f64 {
    let t = 10_f64.powi(n as i32);
    (x * t).round() / t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_to_digits() {
        let a = 123.456789;
        let b = 12345.6789;
        let c = 1.23456789;

        assert_eq!(round_to_digits(a, 2), 123.46);
        assert_eq!(round_to_digits(a, 0), 123.);
        assert_eq!(round_to_digits(a, 6), 123.456789);
        assert_eq!(round_to_digits(b, 1), 12345.7);
        assert_eq!(round_to_digits(b, 3), 12345.679);
        assert_eq!(round_to_digits(b, 4), 12345.6789);
        assert_eq!(round_to_digits(c, 1), 1.2);
        assert_eq!(round_to_digits(c, 2), 1.23);
    }
}
