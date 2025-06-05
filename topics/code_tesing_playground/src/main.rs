

fn main() {
    let s = String::from("hello");
    match s {
        ref val => println!("{}", val),
    }
}
