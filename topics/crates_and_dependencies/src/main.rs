// crates_and_dependencies.rs

// Add this in Cargo.toml:
// fastrand = "2.0"
// or CLI command: cargo add fastrand

use fastrand;
use std::{cmp::max, collections::HashMap, time::Instant};

// 1. Create a binary project using Cargo. Add `fastrand` as a dependency. Verify it builds without error.
fn fastrand_function() -> i32 {
    fastrand::i32(0..=50)
}

// 2. Write a function that returns a random `u8` between 0 and 255 using `fastrand::u8`. Print 10 outputs.
fn print_10_random() -> Vec<u8> {
    (0..10).map(|_| fastrand::u8(0..=255)).collect::<Vec<u8>>()
}

// 3. Generate a `Vec<i32>` of 20 random integers in the range -50 to 50. Count how many are positive, negative, or zero.
fn count_pos_neg() -> (usize, usize, usize) {
    let random20 = (0..20)
        .map(|_| fastrand::i32(-50..=50))
        .collect::<Vec<i32>>();
    let (mut pos, mut neg, mut zero) = (0, 0, 0);
    for i in random20 {
        match i {
            i if i > 0 => pos += 1,
            i if i < 0 => neg += 1,
            _ => zero += 1,
        }
    }
    (pos, neg, zero)
}

// 4. Modify the original `generate_random_number` function to accept a range as parameters. Use it to generate numbers in range 500 to 1000.
fn generate_ranged(range: std::ops::RangeInclusive<i32>) -> i32 {
    fastrand::i32(range)
}

// 5. Write a function that returns the average of 100 random `i32` values in the range 1 to 10.
fn average_100_random() -> i32 {
    let arr = (0..100)
        .map(|_| fastrand::i32(1..=10))
        .collect::<Vec<i32>>();
    (arr.iter().sum::<i32>()) / 100
}

// 6. Use `fastrand::bool()` to simulate 100 coin tosses. Print the count of heads vs tails.
fn coin_toss() -> (usize, usize) {
    let mut true_count: usize = 0;
    let mut false_count: usize = 0;
    let all_toss: Vec<bool> = (0..100).map(|_| fastrand::bool()).collect();
    for toss in all_toss.iter() {
        if *toss == true {
            true_count += 1
        } else {
            false_count += 1
        }
    }
    (true_count, false_count)
}

// 7. Generate 1000 random numbers between 1 and 100. Store the frequency count in a `HashMap<i32, usize>`. Print the most frequent number.
fn count_numbers() -> (i32, usize) {
    let all_random: Vec<i32> = (0..1000).map(|_| fastrand::i32(1..=100)).collect();
    let mut all_numbers: HashMap<i32, usize> = HashMap::new();
    let mut max_freq = 0;

    for &num in all_random.iter() {
        let count = all_numbers.entry(num).or_insert(0);
        *count += 1;
        max_freq = max(max_freq, *count);
    }
    let (&most_number, &most_freq) = all_numbers
        .iter()
        .find(|&(&_key, &val)| val == max_freq)
        .expect("we have the value for sure");

    (most_number, most_freq)
}

// 8. Implement a function that returns a random element from a slice. 
// Use it to randomly choose a string from an array of 5 names.
fn random_element(arg: &[String]) -> String {
    let length_of_arg = (arg.len()) as i32;
    let r_num = (fastrand::i32(0..length_of_arg)) as usize;
    let r_chose = arg[r_num].clone();
    r_chose
}

// 9. Benchmark the runtime of generating 1 million random numbers using `std::time::Instant`. 
// Print elapsed time.
fn benching() -> u128 {
    let initial = Instant::now();
    let _ = (0..1_000_000)
        .map(|_| fastrand::i32(0..100))
        .collect::<Vec<i32>>();
    let finally = Instant::now();
    (finally - initial).as_millis()
}

// 10. Without using `.max()`, implement logic to find the largest value in a vector of random numbers. 
// Write this in a separate function.
fn find_max() -> i32 {
    let random_vec = (0..100).map(|_| fastrand::i32(0..100)).collect::<Vec<i32>>();
    let mut max_num = 0;
    for i in random_vec.iter() {
        if max_num < *i {
            max_num = *i;
        }
    }
    max_num
}

// Bonus test function
fn generate_random_number() -> i32 {
    fastrand::i32(1..=100)
}

fn run() {
    println!("Max random without Max(): {}.", find_max());
    println!("Ex 1: {}", fastrand_function());
    println!("Ex 2: {:?}", print_10_random());
    let (a, b, c) = count_pos_neg();
    println!("Ex 3: Positive: {}, Negative: {}, Zero: {}", a, b, c);
    println!("Ex 4: {}", generate_ranged(500..=1000));
    println!("Ex 5: {}", average_100_random());
    let (a, b) = coin_toss();
    println!("Ex 6: Count of True: {}, Count of False: {}", a, b);
    let (a, b) = count_numbers();
    println!("Ex 7: Number {} appears {} times", a, b);
    let s = [
        "Adarsh".to_string(),
        "Romy".to_string(),
        "Akshay".to_string(),
    ];
    println!(
        "Ex 8: Randomly chosen out of Adarsh, Romy, Akshay: {}",
        random_element(&s)
    );
    println!("Ex 9: Time to generate 1M nums: {} ms", benching());
}

fn main() {
    run();
}