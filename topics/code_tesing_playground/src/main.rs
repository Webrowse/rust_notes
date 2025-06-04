


fn main() {
    let a = Some(5);
    if let Some(ref x) = a {
        println!("{}", x);
    }
    println!("{:?}", a);
}

