// Topic: Variables and Mutability

fn main() {
    // Immutable variable (default)
    let x = 5;
    println!("x = {}", x); 
    // Prints: x = 5

    // x = 6; // Error: cannot assign twice to immutable variable

    // Mutable variable
    let mut y = 10;
    println!("y = {}", y); 
    // Prints: y = 10

    y = 15;
    println!("y = {}", y); 
    // Prints: y = 15

    // Shadowing (redeclare with new value)
    let z = 100;
    let z = z + 1;
    println!("z = {}", z); 
    // Prints: z = 101
}
