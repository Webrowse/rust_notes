// fn main() {
// 1. Match an integer and print custom ranges, exact values, and default case.
// 2. Match an `Option<String>` and handle presence, absence, and content length > 5.
// 3. Use `if let` to extract a `char` from an `Option<char>` and check if it's a vowel.
// 4. Use `while let` on a `Vec<Option<i32>>`, skipping `None`, printing values in reverse.
// 5. Use match guards to categorize an `Option<i32>` as "teen", "adult", or "unknown".
// 6. Destructure nested tuples: `((x, y), z)` and print all three values.
// 7. Define a `struct User { id: u32, name: String }`, destructure it in a match block.
// 8. Match an enum `Shape` with variants `Circle(f64)` and `Rectangle(f64, f64)`, print areas.
// 9. Use `_` to ignore unused fields in tuple and struct destructuring.
// 10. Combine match guards with struct destructuring to filter users older than 18.

// }
fn main() {
    // match with number
    let x = 5;
    match x {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=6 => println!("between 4 and 6"),
        _ => println!("something else"),
    }

    // match with Option
    let maybe = Some(10);
    match maybe {
        Some(n) => println!("got: {}", n),
        None => println!("nothing"),
    }

    // if let for simple Option
    if let Some(v) = maybe {
        println!("if let: {}", v);
    }

    // while let with loop
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("top: {}", top);
    }

    // match guards
    let age = Some(20);
    match age {
        Some(n) if n >= 18 => println!("adult"),
        Some(n) => println!("minor: {}", n),
        None => println!("no age"),
    }

    // destructure tuple
    let pair = (0, -2);
    match pair {
        (0, y) => println!("x is zero, y = {}", y),
        (x, 0) => println!("y is zero, x = {}", x),
        _ => println!("neither is zero"),
    }

    // destructure struct
    struct Point { x: i32, y: i32 }
    let p = Point { x: 1, y: 2 };

    match p {
        Point { x, y } => println!("x = {}, y = {}", x, y),
    }

    // ignore with _
    let (a, _, c) = (1, 2, 3);
    println!("a = {}, c = {}", a, c);
}
