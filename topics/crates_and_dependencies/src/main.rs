// crates_and_dependencies.rs

// Add this in Cargo.toml:
// fastrand = "2.0"

use fastrand;

fn generate_random_number() -> i32 {
    // Generates a number in range [1, 100]
    fastrand::i32(1..=100)
}

fn main() {
    let random = generate_random_number();
    println!("Random number: {}", random);

    // Generate 5 random numbers and find the max
    let nums: Vec<i32> = (0..5).map(|_| fastrand::i32(1..=100)).collect();
    println!("Generated numbers: {:?}", nums);

    if let Some(max) = nums.iter().max() {
        println!("Maximum value: {}", max);
    }
}
