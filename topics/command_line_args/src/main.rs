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

