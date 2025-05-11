// Topic: Enums and Match

fn main() {
    // Defining enum with multiple variants
    let ip1 = IpAddr::V4(127, 0, 0, 1);
    let ip2 = IpAddr::V6(String::from("::1"));

    println!("IPv4: {:?}", ip1); // IPv4: V4(127, 0, 0, 1)
    println!("IPv6: {:?}", ip2); // IPv6: V6("::1")

    // Enum with variants and data
    let msg1 = Message::Quit;
    let msg2 = Message::Write(String::from("hello"));

    msg1.call(); // Output: Quit message
    msg2.call(); // Output: Writing message: hello

    // Option enum
    let some_num = Some(5);
    let no_num: Option<i32> = None;

    println!("Some: {:?}, None: {:?}", some_num, no_num); // Some(5), None

    // Using match
    let coin = Coin::Quarter(UsState::California);
    println!("Coin value: {} cents", value_in_cents(coin)); // 25 cents

    // Matching with Option
    let x = Some(3);
    let y = plus_one(x);
    println!("x+1 = {:?}", y); // Some(4)

    let z = plus_one(None);
    println!("None+1 = {:?}", z); // None
}

// Enum with data
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enum with multiple types of variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Writing message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// Custom enum for matching practice
#[derive(Debug)]
enum UsState {
    Alabama,
    California,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state); // e.g., California
            25
        }
    }
}

// Using Option<T> with match
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n + 1),
        None => None,
    }
}

//Exercises for better understanding:

// 1. enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// fn time_on(light: TrafficLight) -> u32 {
//     match light {
//         TrafficLight::Red => 60,
//         TrafficLight::Yellow => 5,
//         TrafficLight::Green => 30,
//     }
// }

// fn main() {
//     let red = TrafficLight::Red;
//     let yellow = TrafficLight::Yellow;
//     let green = TrafficLight::Green;

//     println!("Red stays on for {} seconds", time_on(red));
//     println!("Yellow stays on for {} seconds", time_on(yellow));
//     println!("Green stays on for {} seconds", time_on(green));
// }

// 2. Create an enum `Shape` with variants `Circle(f64)`, `Rectangle(f64, f64)`, and `Square(f64)`. Write a function that uses `match` to compute the area of each shape.
// enum Shape{
//     Circle(f64),
//     Rectangle(f64, f64),
//     Square(f64),
// }

// impl Shape {
//     fn area(&self) -> f64{
//         match self {
//             Shape::Circle(r) => 3.14*r*r,
//             Shape::Rectangle(l,b) => l*b,
//             Shape::Square(s) => s*s,
//         }
//     }
// }
// fn main(){
//     let c = Shape::Circle(2.0);
//     println!("Area of Circle: {}",c.area());

//     let r = Shape::Rectangle(2.0, 3.0);
//     println!("Area of Rectangle: {}",r.area());

//     let s = Shape::Square(2.0);
//     println!("Area of Square: {}",s.area());
// }
// 3. Implement a method `describe()` for the `IpAddr` enum that returns a string describing the IP address type and value.
// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// impl IpAddr {
//     fn describe(&self) ->String {
//         match self{
//             IpAddr::V4(a,b,c,d) => format!("IP is V4, Value is {}.{}.{}.{}",a,b,c,d),
//             IpAddr::V6(addr) => format!("IP is v6, Value is {}",addr)
//         }
//     }
// }
// fn main(){
//     let test = IpAddr::V4(1,0,0,0);
//     println!("{}",test.describe());
//     let test2 = IpAddr::V6("::1".to_string());
//     println!("{}",test2.describe());
// }
// // 4. Create a function `handle_message(msg: Message)` that takes a `Message` enum and uses a match expression to return a different string based on the variant.
// enum Message {
//     Hello,
//     Hi,
// }

// fn handle_message(msg: Message) -> String {
//     match msg {
//         Message::Hello => format!("hello sir"),
//         Message::Hi => format!("hi ma'am"),
//     }
// }

// fn main() {
//     let a = Message::Hello;
//     let b = handle_message(a);
//     println!("{}", b);
//     println!("{}", handle_message(Message::Hi));
// }

// // 5. Write a match block to convert `Option<String>` into a `String`, using "default" if the `Option` is `None`.
// fn convert(op:Option<String>) -> String{
//     match op{
//         Some(s) => s,
//         None => format!("{}",String::from("default"))
//     }
// }
// fn main (){
// let a = Some(String::from("hellleee"));
// println!("{}",convert(a));
//     let b  = None;
//     println!("{}",convert(b));
// }
// 6. Extend `UsState` with at least three more states.
// Modify `value_in_cents` to print a different message depending on which state the quarter is from.
// enum UsState{
//     Delhi,
//     California,
//     Texas
// }
// fn value_in_cents(state: UsState, curr: i32) -> i32{
//     match state{
//         UsState::Delhi => curr * 2,
//         UsState::California => curr * 3,
//         UsState::Texas => curr * 1,
//     }
// }
// fn main(){
//     let a = UsState::Delhi;
//     let value = value_in_cents(a, 4);
//     println!("{}",value);
// }

// 7. Define a new enum `Result<T, E>` with variants `Ok(T)` and `Err(E)`.
// Write a function that matches on this enum and handles success and error cases differently.

// use std::fmt::format;

// fn result_check<T,E>(res: Result<T,E>) -> String{
//     match res{
//         Ok(_)=> String::from("test is ok: {}"),
//         Err(_)=> String::from("Error is here: {}"),
//     }
// }

// fn main(){
//     let a: Result<i32, &str> = Ok(10);
//     let b: Result<i32, &str> = Err("hehe");

//     println!("{}", result_check(a));
//     println!("{}", result_check(b));
// }

// enum CustomResult<T, E>{
//     Ok(T),
//     Err(E),
// }
// impl<T, E> CustomResult<T, E>{
//     fn check_result(&self) -> String{
//         match self{
//             CustomResult::Ok(_) => String::from("Ok tested"),
//             CustomResult::Err(_) => String::from("Nope error..!"),
//         }
//     }
// }
// fn main(){
//     let a: CustomResult<i32, &str> = CustomResult::Ok(5);
//     let b: CustomResult<i32, &str> = CustomResult::Err("hi");
//     println!("{}",a.check_result());
//     println!("{}", b.check_result());
// }
// 8. Create a function `increment_if_some(opt: Option<i32>) -> Option<i32>` that mimics `plus_one`, but also prints whether a number was incremented or skipped.

// fn increment_if_some(opt: Option<i32>) -> Option<i32>{
//     match opt{
//         Some(mut s) => {
//             s = s+1;
//             println!("Number was increment: {:?}",s);
//             Some(s)
//         },
//         None => {
//             println!("None was passed, so it is skipped");
//             None
//         },
//     }
// }
// fn main(){
//     let a: Option<i32> = Some(49);
//     let b: Option<i32> = None;

//     let result1 = increment_if_some(a);
//     let result2 = increment_if_some(b);

//     let value1 = match result1{
//         Some(v) => v,
//         None => 0,
//     };
//     println!("{}", value1);
// }
// 9. Replace all `match` expressions in the original code with `if let` or `while let` where possible. Explain each transformation in a comment.
// fn increment_if_some(opt: Option<i32>) -> Option<i32> {
//     if let Some(mut s) = opt{
//         s = s + 1;
//         println!("Number increment via if let: {:?}",s);
//         Some(s)
//     } else{
//         println!("None was passed");
//         None
//     }
// }
// fn main() {
//     let a: Option<i32> = Some(49);
//     let b: Option<i32> = None;

//     let result1 = increment_if_some(a);
//     let result2 = increment_if_some(b);

//     if let Some(v) = result1{
//         println!("Value of i32: {}",v)
//     }else{
//         println!("none via if let");
//     }
//     if let Some(v) = result2{
//         println!("value of i32: {}",v);
//     }else{
//         println!("i32 is not found");
//     }
// }
// 10. Write a test function that constructs one of each enum variant in the file, calls their associated functions or match expressions, 
// and verifies expected behavior using `assert_eq!`.

// #[derive(Debug, PartialEq)]
// enum Varient{
//     Number(i32),
//     Data(String),
//     Bolean(bool),
//     StString(&'static str),
//     CustResult(Result<i32,&'static str>),
//     CustOption(Option<i32>)
// }

// fn random_check(dat:Varient) -> String{
//     match dat{
//         Varient::Number(n) => format!("{:?}",n),
//         Varient::Data(s) => format!("{:?}",s),
//         Varient::Bolean(b) => format!("{:?}",b),
//         Varient::StString(st) => format!("{:?}",st),
//         Varient::CustResult( Res) => format!("{:?}",Res),
//         Varient::CustOption(op) => format!("{:?}",op),
//     }
// }
// fn main(){
//  let a = Varient::Number(45);
//  let b =random_check(a);
//  let c = assert_eq!("45",b);
//  println!("succesful tested");

// }