// crates_and_dependencies.rs

// Add this in Cargo.toml:
// fastrand = "2.0"
// or CLI command: cargo add fastrand

use fastrand;

fn _generate_random_number() -> i32 {
    // Generates a number in range [1, 100]
    fastrand::i32(1..=100)
}

fn main() {
   /*
   
   let random = generate_random_number();
   println!("Random number: {}", random);
   
   // Generate 5 random numbers and find the max
   let nums: Vec<i32> = (0..5).map(|_| fastrand::i32(1..=100)).collect();
   println!("Generated numbers: {:?}", nums);
   
   if let Some(max) = nums.iter().max() {
    println!("Maximum value: {}", max);
}
*/ 
println!("Ex 1: \n\t{}",fastrand_function());
println!("Ex 2: \n\t{:?}",print_10_random());
let (a,b,c) = count_pos_neg();
println!("Ex 3: \n\tPositive: {}, Negative: {}, Zero: {}", a,b,c);
println!("Ex 4: \n\t{}",generate_ranged(500..=1000));
}

// 1. Create a binary project using Cargo. Add `fastrand` as a dependency. Verify it builds without error.
fn fastrand_function()-> i32{
     fastrand::i32(0..=50)
}
// 2. Write a function that returns a random `u8` between 0 and 255 using `fastrand::u8`. Print 10 outputs.
fn print_10_random()->Vec<u8>{
    (0..10).map(|_| fastrand::u8(0..=255)).collect::<Vec<u8>>()
}
// 3. Generate a `Vec<i32>` of 20 random integers in the range -50 to 50. Count how many are positive, negative, or zero.
fn count_pos_neg()-> (usize, usize, usize){
    let random20 = (0..20).map(|_| fastrand::i32(-50..=50)).collect::<Vec<i32>>();
    let mut pos = Vec::new(); 
    let mut neg = Vec::new();
    let mut zero = Vec::new(); 
    for i in random20.iter(){
        match *i{
            x if x>0 => pos.push(i),
            x if x<0 => neg.push(i),
            x if x == 0 => zero.push(i),
            _ => unreachable!(),
        }
    }
    (pos.len(),neg.len(),zero.len())
}
// 4. Modify the original `generate_random_number` function to accept a range as parameters. Use it to generate numbers in range 500 to 1000.
fn generate_ranged (range: std::ops::RangeInclusive<i32>) -> i32{
    fastrand::i32(range)
}
// 5. Write a function that returns the average of 100 random `i32` values in the range 1 to 10.

// 6. Use `fastrand::bool()` to simulate 100 coin tosses. Print the count of heads vs tails.

// 7. Generate 1000 random numbers between 1 and 100. Store the frequency count in a `HashMap<i32, usize>`. Print the most frequent number.

// 8. Implement a function that returns a random element from a slice. Use it to randomly choose a string from an array of 5 names.

// 9. Benchmark the runtime of generating 1 million random numbers using `std::time::Instant`. Print elapsed time.

// 10. Without using `.max()`, implement logic to find the largest value in a vector of random numbers. Write this in a separate function.
