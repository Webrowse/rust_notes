// Topic: Borrowing

fn main() {
    // Immutable borrow
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass reference
    println!("Length of '{}' is {}", s1, len); // s1 still valid

    // Mutable borrow
    let mut s2 = String::from("hi");
    change(&mut s2); // pass mutable reference
    println!("Modified s2 = {}", s2);

    // Multiple immutable references
    let s3 = String::from("rust");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1 = {}, r2 = {}", r1, r2); // both valid

    // Mutable reference after immutable ones are done
    let mut s4 = String::from("borrow");
    {
        let r3 = &mut s4;
        r3.push_str("ed");
        println!("r3 = {}", r3);
    } // r3 goes out of scope here
    println!("s4 = {}", s4); // s4 can be used again

    // Dangling reference prevented
    let s5 = no_dangle();
    println!("s5 = {}", s5);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    String::from("safe")
}
