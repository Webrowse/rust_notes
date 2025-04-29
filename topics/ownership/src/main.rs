// Topic: Ownership

fn main() {
    // Ownership Rule 1: Each value has a variable that's its owner

    let s1 = String::from("hello");
    let s2 = s1; // Ownership moves from s1 to s2

    // println!("{}", s1); // ❌ Error: s1 no longer owns the value

    println!("{}", s2); // ✅ s2 owns the value

    // To clone data instead of moving it
    let s3 = String::from("world");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4); // Both are valid

    // Ownership Rule 2: When the owner goes out of scope, value is dropped
    {
        let name = String::from("Adarsh");
        println!("Inside block: {}", name);
    } // name is dropped here

    // Ownership and functions
    let greeting = String::from("Hi");
    takes_ownership(greeting); // greeting is moved

    // println!("{}", greeting); // ❌ Error: greeting moved

    let n = 5;
    makes_copy(n); // i32 is Copy, so still usable

    println!("n still usable: {}", n); // ✅

    // Returning ownership
    let s5 = gives_ownership(); // s5 owns the returned String
    let s6 = String::from("Rust");
    let s7 = takes_and_returns(s6); // s6 is moved in, returned out

    println!("s5 = {}, s7 = {}", s5, s7);
}

fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
}

fn makes_copy(x: i32) {
    println!("Received copy: {}", x);
}

fn gives_ownership() -> String {
    String::from("given")
}

fn takes_and_returns(s: String) -> String {
    s
}
