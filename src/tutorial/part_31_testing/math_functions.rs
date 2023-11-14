pub fn division_with_option_enum(a: f64, b: f64) -> Option<f64> {
    match b {
        0.0f64 => None,
        _ => Some(a / b),
    }
}

pub fn division_with_result_enum(a: f64, b: f64) -> Result<f64, String> {
    match b {
        0.0f64 => Err(String::from("denominator is zero")),
        _ => Ok(a / b),
    }
}

pub fn square(x: i32) -> i32 {
    x * x
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn divide_with_option_enum() {
        let dividend = 5.0;
        let divisor = 2.0;
        assert_eq!(
            division_with_option_enum(dividend, divisor),
            Some(2.5f64),
            "we are testing division with {}/{}",
            dividend,
            divisor
        )
    }

    #[test]
    fn divide_with_result_enum() {
        let dividend = 5.0;
        let divisor = 2.0;
        assert!(
            division_with_option_enum(dividend, divisor).is_some(),
            "we are testing division with {}/{}",
            dividend,
            divisor
        )
    }

    #[test]
    fn square_of_two() {
        // to show this in test run : cargo test -- --show-output
        // to run only this test run : cargo test <test_fn_name>
        println!("2^2={}", square(2));
        assert_eq!(4, square(2));
    }

    #[test]
    fn square_of_three() {
        // to run only this tests with name prefix: cargo test <common_prefix_of_test_names>
        println!("3^2={}", square(3));
        assert_eq!(9, square(3));
    }
}
