// command_line_args.rs

use std::env;
use std::collections::HashSet;
fn main() {
    /*  println!("{:?}",ex1());
    ex2();
    ex3();
    ex4();
    ex5("Hello");
    ex6();
    ex7();    
    println!("exercise 8: {}",ex8());
    println!("Exercise 9: {:?}",ex9());
    */
    ex10();
    /* // Collect arguments into a vector
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
    */
}

// 1. Write a program that counts and prints the number of arguments passed, 
// excluding the program name.
fn ex1() -> usize{
    let args: Vec<String> = env::args().collect();
    return args.len()-1;

}
// 2. Modify the existing code to reverse the order of input arguments 
// (excluding the program name) and print them.
fn ex2(){
    let args: Vec<String> = env::args().skip(1).collect();
    for (i, arg) in args.iter().rev().enumerate(){
        println!("Arguments {}: {}", args.len()-i, arg);
    }   
}
// 3. Implement a version that prints the length of each argument string individually.
fn ex3(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 2{
        for (i, value) in args.iter().enumerate().skip(1){
            println!("length of {}: {}",i,value.len())

        }
    }
}
// 4. Create logic that checks if any argument is a valid integer and prints its square.
fn ex4() {
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args{
        match arg.parse::<i32>(){
            Ok(num) => println!("{}",num * num),
            Err(e) => println!("{}", e)
        }
    }
}
// 5. Add handling to detect and print arguments that contain a specific substring, 
// e.g., "test".
fn ex5(str: &str){
    let args: Vec<String> = env::args().skip(1).collect();
    let found = args.iter().any(|arg| arg.contains(str));
    if found {
        println!("Substring found");
    } else { println!("No substring found")}
}
// 6. Write a program that joins all arguments with a custom delimiter 
// (e.g., `|`) and prints the result.
fn ex6(){
    let args: Vec<String> = env::args().collect();
    let echo = args.iter().skip(1).cloned().collect::<Vec<String>>().join("|");
    println!("{}",echo);
}
// 7. Extend the echo example to convert all input to uppercase before printing.
fn ex7(){
    let args: Vec<String> = env::args().collect();
    let res = args.iter().skip(1).cloned().collect::<Vec<String>>().join(" ").to_uppercase();
    println!("{}",res);
}
// 8. Count how many arguments are purely alphabetic (no digits or symbols).
fn ex8() -> usize{
    let args: Vec<String> = env::args().collect();
    let collect_args: Vec<String> = args.iter().skip(1).cloned().collect();
    let mut pure_alpha_count = 0;
    
    for arg in collect_args{
        let check = arg.chars().all(|c| c.is_alphabetic());
        
        if check {
            pure_alpha_count += 1;
        }
    }
    pure_alpha_count
}
// 9. Detect and print duplicate arguments (case-sensitive) from the input.
fn ex9() -> Vec<String>{
    let args: Vec<String> = env::args().collect();
    let mut hashtable: HashSet<String> = HashSet::new();
    let mut duplicates: Vec<String> = Vec::new();
    
    for entry in args.iter().skip(1).cloned(){
        let success = hashtable.insert(entry.clone());

        if !success{
            duplicates.push(entry);
        }
    }
    duplicates
}
// 10. Transform the program to panic with a custom error message if fewer than two arguments 
// (excluding program name) are given.
fn ex10(){
    let args: Vec<String> = env::args().collect();
    let argu_count = args.iter().skip(1).cloned().collect::<Vec<String>>().len();
    if argu_count < 2 {
        panic!("arguments less than 2");
    };

}

