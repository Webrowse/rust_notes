// basic_unit_testing.rs

//  Regular function to test
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//  Private logic for test coverage
pub fn is_even(n: i32) -> bool {  //made public for ex7
    n % 2 == 0
}

//  Tests go in a special `#[cfg(test)]` module
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
#[cfg(test)]
mod ex1{
    use super::*;
    #[test]
    fn test_add(){
        assert_eq!(add(0,0),0);
    }
}
fn main(){}
// 2. Write a test for `add(i32::MAX, 1)` and check for overflow behavior.
#[cfg(test)]
mod ex2{
    use std::i32;

    use super::add;
    #[test]
    #[should_panic]
    fn test_add(){
        assert_eq!(add(i32::MAX,1),23);
    }
}
// 3. Refactor `is_even` to handle negative values and test `is_even(-4)` and `is_even(-3)`.
#[cfg(test)]
mod ex3{
    use super::is_even;
    #[test]
    fn test_negetive_value_in_is_even(){
        assert_eq!(is_even(-4), true);
    }

}
// 4. Add a test for `add(a, b)` where `a` is positive and `b` is negative, verify correctness.
#[cfg(test)]
mod ex4{
    use super::add;
    #[test]
    fn testing_add_negetive_values(){
        assert_eq!(add(8, -5),3);

    }
}
// 5. Test `add(100, -100)` and verify it returns `0`.
#[cfg(test)]
mod ex5{
    use super::add;
    #[test]
    fn test_add_for_100(){
        assert_eq!(add(100,-100),0);
    }
}
// 6. Remove `#[should_panic]` from `test_failure` and observe test failure behavior.
#[cfg(test)]
mod ex6{
    use super::add;
    #[test]
    fn intent_panic(){
        assert_eq!(add(10,12),31);
    }
}
// 7. Move `is_even` to a public function and test it from an external module.
#[cfg(test)]
mod ex7{
    use crate::is_even;

    #[test]
    fn access_from_pub_fun(){
        assert_eq!(is_even(2),true);
    }
}
// 8. Write a helper function that asserts a range of numbers are even or odd, and test them in a loop.
#[cfg(test)]
mod helper{
    use super::is_even;
    #[test]
    fn range_of_numbers(){
        let mut even: Vec<i32> = Vec::new();
        for i in 1..5{
            if is_even(i){
                even.push(i);
            }
        }
        assert_eq!(vec![2,4],even);
    }
}
// 9. Add `#[ignore]` to a test and verify it's skipped by default in `cargo test`.
#[cfg(test)]
mod ignoing{
    use super::add;
    #[test]
    #[ignore]
    fn test_ignore(){
        assert_eq!(add(34,4),6)
    }
}
// 10. Use `assert_ne!` instead of `assert_eq!` in one test and verify opposite logic.
#[cfg(test)]
mod ex10{
    use super::add;
    #[test]
    fn test_assert_nq(){
        assert_ne!(add(2,3),4);
    }
}