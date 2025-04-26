// Topic: Functions and Statements

// Function with no parameters and no return
fn greet() {
    println!("Hello from greet()");
}

// Function with parameters
fn add(a: i32, b: i32) {
    println!("Sum is: {}", a + b);
}

// Function with return value
fn square(x: i32) -> i32 {
    x * x // last expression is the return value
}

// Function with early return
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    false
}

// Statements vs Expressions
fn statements_vs_expressions() {
    // Statements: do something but return nothing
    let x = 5; // statement

    // Expressions: evaluate to a value
    let y = {
        let temp = 3;
        temp + 2 // no semicolon → this is returned
    };

    println!("x = {}, y = {}", x, y); // y = 5
}

fn main() {
    greet(); // Prints: Hello from greet()

    add(4, 5); // Prints: Sum is: 9

    let sq = square(6);
    println!("Square is: {}", sq); // Prints: 36

    println!("Is 10 even? {}", is_even(10)); // Prints: true
    println!("Is 7 even? {}", is_even(7)); // Prints: false

    statements_vs_expressions();
}
