use std::char;

fn main2() {
// 1. Match an integer and print custom ranges, exact values, and default case.
let a: i32 = 5;
match a {
    1 => println!("a is {a}"),
    2|3 => println!("a is 2 or 3"),
    4..6 => println!("a is between 4 to 5"),
    _ => println!("a is something outside 1 to 5"),
}
// 2. Match an `Option<String>` and handle presence, absence, and content length > 5.
let a: Option<String> = Some("Exercise.2".to_string());
match a {
    // Some(n) => println!("String is present, and content length is {:?}",n.len()),
    Some(ref n) if n.len()>5 => println!("Long String: {n}"),
    Some(ref n) => println!("Short String: {n}"),
    None => println!("Absent")
}
// 3. Use `if let` to extract a `char` from an `Option<char>` and check if it's a vowel.
let vowel: Option<char> =Some('b');
if let Some(b)= vowel{
    match b{
        'a'|'e'|'i'|'o'|'u' => println!("Vowel: {b}"),
        _ => println!("No vowels")
    }
}
// 4. Use `while let` on a `Vec<Option<i32>>`, skipping `None`, printing values in reverse.
let mut ex4 = vec![Some(1), Some(4), None, Some(3)];
while let Some(n) = ex4.pop(){
    match n{
        Some(val) => println!("{:?}",val),
        None => (),
    }
}
// 5. Use match guards to categorize an `Option<i32>` as "teen", "adult", or "unknown".
let age: Option<i32> = Some(34);
match age{
    Some(n) if n < 13 => println!("Preteen: {:?}",n), 
    Some(n) if n > 12 && n < 20 => println!("teen: {:?}",n), 
    Some(n) if n > 19 => println!("adult: {:?}",n), 
    Some(_) => println!("Any other age"),
    None => println!("Invalid data"), 
}

// 6. Destructure nested tuples: `((x, y), z)` and print all three values.
let ((x,y),z) = ((2,3),4);
println!("{},{},{}",x,y,z);

// 7. Define a `struct User { id: u32, name: String }`, destructure it in a match block.
struct User{ id: u32, name: String }
let user: User = User{ id: 1, name: String::from("Hello there")};
match user{
    User{id,name} => println!("{:?}, {:?}",id,name)
}
// 8. Match an enum `Shape` with variants `Circle(f64)` and `Rectangle(f64, f64)`, print areas.
enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
}
let sha: Shape = Shape::Circle(2.0);
match sha{
    Shape::Circle(n) => println!("{}", std::f64::consts::PI * n * n),
    Shape::Rectangle(l,m) => println!("{}", l * m)
}

// 9. Use `_` to ignore unused fields in tuple and struct destructuring.
// 10. Combine match guards with struct destructuring to filter users older than 18.
println!("****************************");
}
fn main() {
    main2();
    // match with number
    // let x = 5;
    // match x {
    //     1 => println!("one"),
    //     2 | 3 => println!("two or three"),
    //     4..=6 => println!("between 4 and 6"),
    //     _ => println!("something else"),
    // }

    // // match with Option
    // let maybe = Some(10);
    // match maybe {
    //     Some(n) => println!("got: {}", n),
    //     None => println!("nothing"),
    // }

    // // if let for simple Option
    // if let Some(v) = maybe {
    //     println!("if let: {}", v);
    // }

    // // while let with loop
    // let mut stack = vec![1, 2, 3];
    // while let Some(top) = stack.pop() {
    //     println!("top: {}", top);
    // }

    // // match guards
    // let age = Some(20);
    // match age {
    //     Some(n) if n >= 18 => println!("adult"),
    //     Some(n) => println!("minor: {}", n),
    //     None => println!("no age"),
    // }

    // // destructure tuple
    // let pair = (0, -2);
    // match pair {
    //     (0, y) => println!("x is zero, y = {}", y),
    //     (x, 0) => println!("y is zero, x = {}", x),
    //     _ => println!("neither is zero"),
    // }

    // // destructure struct
    // struct Point { x: i32, y: i32 }
    // let p = Point { x: 1, y: 2 };

    // match p {
    //     Point { x, y } => println!("x = {}, y = {}", x, y),
    // }

    // // ignore with _
    // let (a, _, c) = (1, 2, 4);
    // println!("a = {}, c = {}", a, c);
}
