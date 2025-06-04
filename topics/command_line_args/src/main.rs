// command_line_args.rs

use std::env;

fn main() {
    // Collect arguments into a vector
    let args: Vec<String> = env::args().collect();

    println!("All arguments: {:?}", args);

    if args.len() < 2 {
        println!("Please provide at least one argument.");
        return;
    }

    // Access individual arguments
    let program_name = &args[0]; // Usually the executable name
    println!("Program name: {}", program_name);

    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("Argument {}: {}", i, arg);
    }

    // Example: simple echo
    let echo: String = args.iter().skip(1).cloned().collect::<Vec<String>>().join(" ");
    println!("Echoed: {}", echo);
}

// 1. Write a program that counts and prints the number of arguments passed, 
// excluding the program name.

// 2. Modify the existing code to reverse the order of input arguments 
// (excluding the program name) and print them.

// 3. Implement a version that prints the length of each argument string individually.

// 4. Create logic that checks if any argument is a valid integer and prints its square.

// 5. Add handling to detect and print arguments that contain a specific substring, 
// e.g., "test".

// 6. Write a program that joins all arguments with a custom delimiter 
// (e.g., `|`) and prints the result.

// 7. Extend the echo example to convert all input to uppercase before printing.

// 8. Count how many arguments are purely alphabetic (no digits or symbols).

// 9. Detect and print duplicate arguments (case-sensitive) from the input.

// 10. Transform the program to panic with a custom error message if fewer than two arguments 
// (excluding program name) are given.


