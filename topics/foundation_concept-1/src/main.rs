// // Topic: Foundational Concepts in Rust

// fn main() {
//     // --------- Shadowing ----------
//     let x = 5;
//     let x = x + 1; // shadowed
//     let x = x * 2; // shadowed again
//     println!("Shadowed x: {}", x); // 12

//     // --------- Type Inference & Annotation ----------
//     let inferred = 10; // inferred as i32
//     let annotated: u64 = 20; // explicitly annotated
//     println!("Inferred: {}, Annotated: {}", inferred, annotated); // 10, 20

//     // --------- Expressions vs Statements ----------
//     let y = {
//         let inner = 3;
//         inner + 1 // expression — returns value
//     };
//     println!("Expression result: {}", y); // 4

//     // Statements do *not* return a value
//     // let z = let a = 5; // ERROR: statements don’t return values

//     // --------- Destructuring ----------
//     let tup = (1, "hello", true);
//     let (a, b, c) = tup;
//     println!("Tuple values: {}, {}, {}", a, b, c); // 1, hello, true

//     let user = User {
//         username: String::from("adarsh"),
//         email: String::from("adarsh@example.com"),
//         active: true,
//     };

//     // Struct destructuring with shorthand
//     let User { username, email, active } = user;
//     println!("Destructured: {} - {} - {}", username, email, active); // values from struct

//     // --------- Default Trait ----------
//     let config = Config::default(); // uses default values
//     println!("Timeout: {}, Retries: {}", config.timeout, config.retries); // 30, 3

//     // --------- dbg! Macro ----------
//     let debug_var = 42;
//     dbg!(debug_var); // prints to stderr: [src/main.rs:48] debug_var = 42

//     let calc = dbg!(5 + 3 * 2); // prints intermediate result: 11
//     println!("Calc result: {}", calc); // 11
// }

// // Struct for destructuring
// struct User {
//     username: String,
//     email: String,
//     active: bool,
// }

// // Struct with Default trait
// #[derive(Debug)]
// struct Config {
//     timeout: u32,
//     retries: u8,
// }

// impl Default for Config {
//     fn default() -> Self {
//         Config {
//             timeout: 30,
//             retries: 3,
//         }
//     }
// }

// Exercises for Understanding

// 1. Write a function where a variable is shadowed three times, each altering its type. Print the final value and type.

// fn main(){
//     let x = 23;
//     println!("{}",x);
//     let x = "hi";
//     println!("{}",x);
//     let x = true;
//     println!("{}",x);
// }

// 2. Declare five variables with interred types, then rewrite them with explicit type annotations.

// fn main(){
//     let infer1 = 23;
//     let infer2 = true;
//     let infer3 = "hi";
//     let infer4 = String::from("Hello");
//     let infer5 = 3.14;

//     println!("infered: {}, {}, {}, {}, {}",infer1, infer2,infer3,infer4,infer5);

//     let anno1: i8 = 23;
//     let anno2: bool = true;
//     let anno3: &str = "hi";
//     let anno4: String = String::from("Hello");
//     let anno5: f32 = 3.14;

//     println!("Annoted: {}, {}, {}, {}, {}",anno1, anno2, anno3, anno4, anno5);
// }

// 3. Create a block that returns a value using an expression. Assign it to a variable and print the result.
// Then attempt to assign a block ending in a statement and observe the compiler error.

// fn main(){
//     let x = {
//         let i = 23;
//         i/3
//     };
//     let y = x;
//     println!("{y}");

//     let z = if x == 7 { 3}else {2};
//     println!("{z}");
//     let v = let u = 4;
//     println!("error: {}",v);
// }

// 4. Create a nested tuple and destructure it into individual variables. print each variable.

// fn main(){
//     let a = (49,"hi",true);
//     let (a , b , c) = a;
//     println!("{a}, {b}, {c}");

//     let x = ((4,true,"hi"),(6, false),(8, false));
//     let (p,q) = x.1;
//     let ((a1, a2, a3),(b1, b2), (c1, c2)) = x;
//     println!("{a1},{a2},{a3},{b1},{b2},{c1},{c2}");
//     println!("{}, {}",p,q);
// }

// 5. Define a struct with four fields. instantiate it and destructure it using both shorthand and full syntax. Print the fields.

// fn main(){
//     struct Four{
//         first_name: String,
//         last_name: String,
//         age: i32,
//         is_active: bool,
//     }
//     let user = Four{
//         first_name: "Romy".to_string(),
//         last_name: "iiieee".to_string(),
//         age: 90,
//         is_active: true,
//     };
//     let Four {age:a, first_name:fnm, last_name:lnm, is_active:ia}  = user;
//     println!("{},{},{},{}",a,fnm,lnm,ia);
// }

//6. implement a Default trait for a new struct representing user preferences. Use .default() and print all default fields.
// #[derive(Debug)]
// struct Testing{
//     attempt: u8,
//     tired: bool,
// }
// impl Default for Testing{
//     fn default() -> Self {
//         Testing{
//             attempt: 1,
//             tired: false,
//         }
//     }
// }
// fn main(){
//     let test1 = Testing{
//         attempt: 2,
//         tired: true,
//     };
//     println!("{:?}",test1);
//     let test2 = Testing::default();
//     println!("{:?}",test2);
// }

// 7. Use dbg! to trace the flow of a math expression across three steps: input variables, intermediate operation, and final result.
// fn main(){
//     //input variable
//     let x = 6;
//     dbg!(x);
//     // intermediate operations
//     dbg!(true&&false);
//     dbg!(3.14*2.43);
//     //final result
//     let a = dbg!({
//         let b = 4;
//         b*6/2
//     });
//     dbg!(a);
// }

// 8. Combine shadowing and type annotation in a loop.
// Change the type of a loop variable inside the loop and explain why it does or doesn't compile.
// fn main(){
    // let x = 43;
    // for i in 0..3{
    //     let x: String=  match i{ //Match should have same return type
    //         0=> {
    //             let x:&str = "Romy";
    //             println!("{}",x);
    //             x.to_string()        
//             }
//             1=> {
//                 let x:bool = true;
//                 println!("{}",x);
//                 x.to_string()
//             }
//             2=> {
//                 let x=String::from("hello");
//                 println!("{}",x);
//                 x.to_string()
//             }
//             _=>x.to_string()
//         };
//         println!("Inner x: {}",x);
//     }
//     println!("Outer: {}",x);
// }
// 9. Build a function that accepts a struct, destructures it in parameters, and returns a strings combining its fields
// struct Exer{
//     name:&'static str,
//     age: i8,
// }
// fn for_struct(stt: Exer) -> String{
//     let Exer{name, age} = stt;
//     format!("{} is {} years old",name,age)
    
// }
// fn main(){
//     let user = Exer{
//         name:"romy",
//         age: 17,
//     };
//     let res = for_struct(user);
//     println!("Combined String: {}",res);
// }

// 10. Use a dbg! macro inside a custom function to inspect both arguments and return value. 
// Call this function in main and print the final result.

fn trial (arg: &str) -> String{
    dbg!(arg);
 let s = arg.to_string();
 println!("{}",s);
 dbg!(s.clone());
 s
}
fn main(){
    let x = "hello";
    let y = trial(x);
    println!("success: {}",y);
}

