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

// tests

// 1. Write a test for `add(0, 0)` and verify the result.

// 2. Write a test for `add(i32::MAX, 1)` and check for overflow behavior.

// 3. Refactor `is_even` to handle negative values and test `is_even(-4)` and `is_even(-3)`.

// 4. Add a test for `add(a, b)` where `a` is positive and `b` is negative, verify correctness.

// 5. Test `add(100, -100)` and verify it returns `0`.

// 6. Remove `#[should_panic]` from `test_failure` and observe test failure behavior.

// 7. Move `is_even` to a public function and test it from an external module.

// 8. Write a helper function that asserts a range of numbers are even or odd, and test them in a loop.

// 9. Add `#[ignore]` to a test and verify it's skipped by default in `cargo test`.

// 10. Use `assert_ne!` instead of `assert_eq!` in one test and verify opposite logic.
