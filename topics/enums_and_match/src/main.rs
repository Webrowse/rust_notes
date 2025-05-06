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

// 3. Implement a method `describe()` for the `IpAddr` enum that returns a string describing the IP address type and value.

// 4. Create a function `handle_message(msg: Message)` that takes a `Message` enum and uses a match expression to return a different string based on the variant.

// 5. Write a match block to convert `Option<String>` into a `String`, using "default" if the `Option` is `None`.

// 6. Extend `UsState` with at least three more states. Modify `value_in_cents` to print a different message depending on which state the quarter is from.

// 7. Define a new enum `Result<T, E>` with variants `Ok(T)` and `Err(E)`. Write a function that matches on this enum and handles success and error cases differently.

// 8. Create a function `increment_if_some(opt: Option<i32>) -> Option<i32>` that mimics `plus_one`, but also prints whether a number was incremented or skipped.

// 9. Replace all `match` expressions in the original code with `if let` or `while let` where possible. Explain each transformation in a comment.

// 10. Write a test function that constructs one of each enum variant in the file, calls their associated functions or match expressions, and verifies expected behavior using `assert_eq!`.

