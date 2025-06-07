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

// 1. Create a binary project using Cargo. Add `fastrand` as a dependency. Verify it builds without error.

// 2. Write a function that returns a random `u8` between 0 and 255 using `fastrand::u8`. Print 10 outputs.

// 3. Generate a `Vec<i32>` of 20 random integers in the range -50 to 50. Count how many are positive, negative, or zero.

// 4. Modify the original `generate_random_number` function to accept a range as parameters. Use it to generate numbers in range 500 to 1000.

// 5. Write a function that returns the average of 100 random `i32` values in the range 1 to 10.

// 6. Use `fastrand::bool()` to simulate 100 coin tosses. Print the count of heads vs tails.

// 7. Generate 1000 random numbers between 1 and 100. Store the frequency count in a `HashMap<i32, usize>`. Print the most frequent number.

// 8. Implement a function that returns a random element from a slice. Use it to randomly choose a string from an array of 5 names.

// 9. Benchmark the runtime of generating 1 million random numbers using `std::time::Instant`. Print elapsed time.

// 10. Without using `.max()`, implement logic to find the largest value in a vector of random numbers. Write this in a separate function.
