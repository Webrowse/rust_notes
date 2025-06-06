// basic_unit_testing.rs

// 🔹 Regular function to test
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 🔹 Private logic for test coverage
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// 🔹 Tests go in a special `#[cfg(test)]` module
#[cfg(test)]
mod tests {
    // Import names from outer scope
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_is_even_true() {
        assert!(is_even(4));
    }

    #[test]
    fn test_is_even_false() {
        assert!(!is_even(3));
    }

    #[test]
    #[should_panic]
    fn test_failure() {
        panic!("this test should panic");
    }
}
