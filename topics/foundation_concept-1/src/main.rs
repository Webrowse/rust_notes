// Topic: Foundational Concepts in Rust

fn main() {
    // --------- Shadowing ----------
    let x = 5;
    let x = x + 1; // shadowed
    let x = x * 2; // shadowed again
    println!("Shadowed x: {}", x); // 12

    // --------- Type Inference & Annotation ----------
    let inferred = 10; // inferred as i32
    let annotated: u64 = 20; // explicitly annotated
    println!("Inferred: {}, Annotated: {}", inferred, annotated); // 10, 20

    // --------- Expressions vs Statements ----------
    let y = {
        let inner = 3;
        inner + 1 // expression — returns value
    };
    println!("Expression result: {}", y); // 4

    // Statements do *not* return a value
    // let z = let a = 5; // ERROR: statements don’t return values

    // --------- Destructuring ----------
    let tup = (1, "hello", true);
    let (a, b, c) = tup;
    println!("Tuple values: {}, {}, {}", a, b, c); // 1, hello, true

    let user = User {
        username: String::from("adarsh"),
        email: String::from("adarsh@example.com"),
        active: true,
    };

    // Struct destructuring with shorthand
    let User { username, email, active } = user;
    println!("Destructured: {} - {} - {}", username, email, active); // values from struct

    // --------- Default Trait ----------
    let config = Config::default(); // uses default values
    println!("Timeout: {}, Retries: {}", config.timeout, config.retries); // 30, 3

    // --------- dbg! Macro ----------
    let debug_var = 42;
    dbg!(debug_var); // prints to stderr: [src/main.rs:48] debug_var = 42

    let calc = dbg!(5 + 3 * 2); // prints intermediate result: 11
    println!("Calc result: {}", calc); // 11
}

// Struct for destructuring
struct User {
    username: String,
    email: String,
    active: bool,
}

// Struct with Default trait
#[derive(Debug)]
struct Config {
    timeout: u32,
    retries: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timeout: 30,
            retries: 3,
        }
    }
}
