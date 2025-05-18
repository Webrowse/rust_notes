// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
//     // Option: maybe something, maybe nothing
//     let some_number = Some(10);
//     let no_number: Option<i32> = None;

//     if let Some(n) = some_number {
//         println!("Got number: {}", n);
//     }

//     match no_number {
//         Some(n) => println!("Number: {}", n),
//         None => println!("No number"),
//     }

//     // unwrap_or
//     let x = no_number.unwrap_or(100);
//     println!("unwrap_or: {}", x); // 100

//     // Result: Ok or Err
//     let f = File::open("demo.txt");

//     match f {
//         Ok(file) => println!("File opened: {:?}", file),
//         Err(e) => println!("Failed to open: {}", e),
//     }

//     // unwrap_or_else
//     let fallback = File::open("demo.txt").unwrap_or_else(|_| {
//         File::create("demo.txt").expect("create failed")
//     });
//     println!("file ready: {:?}", fallback);

//     // ? operator (works in functions returning Result)
//     match read_file_contents("demo.txt") {
//         Ok(contents) => println!("File contents:\n{}", contents),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn read_file_contents(path: &str) -> Result<String, io::Error> {
//     let mut f = File::open(path)?; // returns early on Err
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)?;
//     Ok(contents)
// }



// 1. Create an `Option<String>` with some value, then print the inner string only if it exists.
fn main(){
    let first:Option<i32> = Some(3);
    if let Some(n) = first{
        println!("Exercise 1: first is: {:?}",Some(n));
    }


// 2. Create a function `double_if_some(x: Option<i32>) -> Option<i32>` that returns double the value if `Some`, else `None`.
fn double_if_some(x: Option<i32>) -> Option<i32>{
    match x{
        Some(x) => Some(2 * x),
        None => None
    }
}
let second = double_if_some(Some(4));
println!("Exercise 2: {:?}",second);

// 3. Use `unwrap_or` on `Option<&str>` to provide a default string if `None`.
let s:Option<&str> = None;
let t = s.unwrap_or("hi");
println!("Exercise 3: {:?}",t);
// 4. Use `match` on an `Option<i32>` to add 5 if it exists, else subtract 1 from zero.
let a:Option<i32> = Some(2);
let b = match a{
    Some(n)=> {
        println!("Exercise 4: {:?}",n+5);
        n+5
    }
    None => {
        println!("Exercise 4: -1");
        -1
    }
};


// 5. Attempt to open a non-existent file with `File::open` and handle it using `match`.

// 6. Write a function `open_or_create(path: &str) -> File` using `unwrap_or_else` to create the file if it doesn’t exist.
// 7. Write a function `read_lines(path: &str) -> Result<Vec<String>, io::Error>` that reads all lines from a file using `?`.
// 8. Create a vector of `Option<i32>`, then filter and sum all `Some` values.
// inefficient way
// let vo: Vec<Option<i32>> = vec![Some(2),Some(4),None, Some(2)];
// let sumvo: Vec<_> = vo.into_iter().filter(|n|*n!=None).collect();
// let normal: Vec<_> = sumvo.into_iter().flatten().collect();
// let ex8 = normal.iter().fold(0,|acc, n| acc + *n);
// println!("{}",ex8);
// efficient way:-
let vo: Vec<Option<i32>> = vec![Some(2),Some(3), None, Some(4)];
let sum_of_option: i32 = vo.iter().flatten().sum();
println!("exercise 8: {}",sum_of_option);

// 9. Write a function `safe_divide(a: i32, b: i32) -> Option<i32>` that returns `None` if `b` is zero.
println!("exercise 9: a=2, b=0: {:?}",safe_divide(2, 0));
println!("          : a=2, b=2: {:?}",safe_divide(2, 2));
// 10. Write a function `try_append(path: &str, text: &str) -> Result<(), io::Error>` that appends text to a file or returns the error.

}
fn safe_divide(a:i32, b:i32) -> Option<i32>{
    // let res = match b{
    //     0  => None,
    //     _ => Some(a / b),
    //     };
    //     res
    if b == 0 {None} else {Some(a/b)}
}