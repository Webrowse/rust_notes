fn main() {
    fn even_vec(v: Vec<i32>) -> Vec<i32> {
        v.iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, value)| *value)
            .collect()
    }
    println!(
        "exercise 9: {:?}",
        even_vec(vec![1, 54, 23, 56, 44, 13, 22])
    );
}
