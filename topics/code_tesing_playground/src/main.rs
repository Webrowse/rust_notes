fn main() {
    let s = String::from("hello");
    let opt_ref: Option<&String> = Some(&s);
    
    let opt_cloned: Option<String> = opt_ref.cloned();  // Clones the inner `String`

    println!("{:?}", opt_cloned);
    println!("{:?}", opt_ref); 

    // Ownership check:
    println!("{}", s); // Still accessible, not moved
}
