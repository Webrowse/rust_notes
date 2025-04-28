// Topic: Control Flow

fn main() {
    // if and else
    let number = 7;

    if number < 5 {
        println!("Less than 5");
    } else if number == 7 {
        println!("Exactly 7");
    } else {
        println!("Greater than 5");
    }

    // loop (infinite unless broken)
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Reached 3, breaking");
            break;
        }
    }

    // while loop
    let mut num = 5;
    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }
    println!("LIFTOFF!");

    // for loop with range
    for i in 1..4 {
        println!("i = {}", i); // Prints 1, 2, 3 (4 not included)
    }

    // for loop over array
    let arr = [10, 20, 30];
    for val in arr {
        println!("val = {}", val);
    }

    // break and continue
    for n in 0..5 {
        if n == 2 {
            println!("Skipping 2");
            continue;
        }
        if n == 4 {
            println!("Breaking at 4");
            break;
        }
        println!("n = {}", n);
    }

    // match
    let direction = "left";

    match direction {
        "left" => println!("Go left"),
        "right" => println!("Go right"),
        _ => println!("Go straight"),
    }

    // match with number
    let score = 85;
    match score {
        90..=100 => println!("Excellent"),
        70..=89 => println!("Good"),
        0..=69 => println!("Needs Improvement"),
        _ => println!("Invalid score"),
    }
}
