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
