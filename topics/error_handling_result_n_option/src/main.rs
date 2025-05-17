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
    let first2:Option<i32> = None;

    if let Some(n) = first{
        println!("first is {:?}, first2 is {:?}",first,first2);
    }


// 2. Create a function `double_if_some(x: Option<i32>) -> Option<i32>` that returns double the value if `Some`, else `None`.


// 3. Use `unwrap_or` on `Option<&str>` to provide a default string if `None`.
// 4. Use `match` on an `Option<i32>` to add 5 if it exists, else subtract 1 from zero.
// 5. Attempt to open a non-existent file with `File::open` and handle it using `match`.
// 6. Write a function `open_or_create(path: &str) -> File` using `unwrap_or_else` to create the file if it doesn’t exist.
// 7. Write a function `read_lines(path: &str) -> Result<Vec<String>, io::Error>` that reads all lines from a file using `?`.
// 8. Create a vector of `Option<i32>`, then filter and sum all `Some` values.
// 9. Write a function `safe_divide(a: i32, b: i32) -> Option<i32>` that returns `None` if `b` is zero.
// 10. Write a function `try_append(path: &str, text: &str) -> Result<(), io::Error>` that appends text to a file or returns the error.

}