// lifetimes_basics.rs
// 1. Implement `longest_of_three<'a>(a: &'a str, b: &'a str, c: &'a str) -> &'a str`
//    Use conditional logic to return the longest string among three.
    fn longest_of_three<'a>(a: &'a str, b: &'a str, c:&'a str) -> &'a str {
        let mut longest = a;
        if b.len()>longest.len(){ longest = b}
        if c.len()>longest.len(){ longest = c}
        longest
        // if a.len() > b.len() && a.len() > c.len(){ a }
        // else if b.len() > c.len() && b.len() > a.len() { b }
        // else { c }
    }
    fn exercise1(){
        let x = String::from("hi");
        let y = String::from("hiiiiiii");
        let z = String::from("hello");
        let res = longest_of_three(&x, &y, &z);
        println!("{}",res);
    }
// 2. Create `shortest<'a>(x: &'a str, y: &'a str) -> &'a str`
//    Mimic the logic of `longest` but return the shortest string.
fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() { x } else { y }
}
fn exercise2(){
    let a = "yes";
    let b = "yeah";
    let c = shortest(&a, &b);
    println!("{}",c);
}
// 3. Define `struct Label<'a> { text: &'a str }`
//    Write a function `print_label(label: &Label)` that prints `label.text`.
#[derive(Debug)]
struct Label<'a>{
    text: &'a str
}
fn print_label<'a>(label: &Label<'a>){
    println!("{}",label.text);
}
fn exercise3(){
    let la = Label{text: "hello"};
    print_label(&la);
}
// 4. Write a function `wrap_in_excerpt<'a>(text: &'a str) -> Label<'a>`
//    Return an `ImportantExcerpt` with the input string as the field.
fn wrap_in_excerpt<'a>(text: &'a str) -> Label<'a>{
    Label{text: text}
}
fn exercise4(){
    let tx = "forth exercise".to_string();
    println!("{:?}", wrap_in_excerpt(&tx).text);
}
// 5. Attempt to return a reference to a local string inside a function
//    Observe the compiler error: `returns a reference to data owned by the current function`.
// fn not_working<'a>() -> &'a str{
//     let data = String::from("return this, observe errors");
//     &data
// }
// Its impossible to return a reference of a local data from a function.
// fn exercise5(){
//     let a = not_working();
//     println!("{}",a);
// }
// 6. Implement `compare_and_return<'a>(x: &'a str, y: &'a str) -> &'a str`
//    Include an inner block where one input is declared. Ensure no compile error.
fn compare_and_return<'a>(x: &'a str, y: &'a str) -> &'a str{
    let res = if x.len()>y.len(){ x } else { y };
    res
}
fn exercise6(){
    let a = String::from("short_String");
    {
        let b = String::from("long_string");
        println!("{}",compare_and_return(&a, &b));
    }
}
// 7. Write a function `split_and_return<'a>(s: &'a str) -> &'a str`
//    Return the first sentence from the input string using `.split('.')`.
fn split_and_return<'a>(s: &'a str) -> &'a str{
      s.split(".").next().unwrap()
}
fn exercise7(){
    let sentence = String::from("This is me. Hi. How are you?");
    let res = split_and_return(&sentence);
    println!("{}",res);
}
// 8. Pass a string literal and a `String` reference to `longest`
//    Observe type compatibility between `&'static str` and `&String`.
fn somefunction(s: &str, t: &String){
    println!("{},{}",s,t);

}
fn exercise8(){
    let a = "stringLiteral";
    let b = "String".to_string();
    somefunction(&a, &b);
}
// 9. Build `struct Container<'a> { item: Option<&'a str> }`
//    Implement a method `get(&self) -> Option<&'a str>`.
struct Container<'a>{
    item: Option<&'a str>
}

impl<'a>  Container<'a>{
    fn get(&self) -> Option<&'a str>{
        self.item
    }
}
fn exercise9(){
    let text = String::from("some data");
    let c = Container { item: Some(&text)};
    if let Some(val) = c.get(){
        println!("{}", val);
    }
}
// 10. Manually cause a dangling reference by creating a reference in an inner block and returning it
//     Force a lifetime error. Refactor to fix it using proper scopes.
fn manually<'a>(a: &'a str) -> &'a str{
     a
}
fn exercise10(){
    let input = String::from("exercise 10 fixed");
    let error_output = manually(&input);
    println!("{}",error_output);
}

// ----------- Example 1: function that returns one of two string references -----------

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // both x and y must live as long as lifetime 'a
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn example_1() {
    let s1 = String::from("short");
    let s2 = String::from("a bit longer");
    let result = longest(&s1, &s2);
    println!("Longest: {}", result); // prints: a bit longer
}

// ----------- Example 2: lifetime prevents returning a dangling reference -----------

// This won't compile:
// fn invalid_ref() -> &str {
//     let s = String::from("oops");
//     &s // ❌ error: s is dropped at end of function
// }

// ----------- Example 3: Structs holding references need lifetime annotations -----------

struct Book<'a> {
    title: &'a str,
}

fn example_2() {
    let t = String::from("Rust in Action");
    let b = Book { title: &t };
    println!("Book title: {}", b.title); // prints: Rust in Action
}

// ----------- Example 4: Lifetime elision rules -----------

// In many cases, Rust can infer lifetimes and you don’t have to write them explicitly.

fn _print_str(s: &str) {
    // works fine without specifying lifetimes
    println!("{}", s);
}

// ----------- Example 5: Static lifetime -----------

// 'static means the data lives for the entire program duration.
fn static_str() -> &'static str {
    "I live forever"
}

// ----------- Example 6: Returning a reference to local variable (❌ will not compile) -----------

// fn invalid() -> &String {
//     let s = String::from("temp");
//     &s // ❌ error: s doesn't live long enough
// }

// ----------- Entry point -----------

fn main() {
    example_1();
    example_2();
    println!("{}", static_str());
    exercise1();
    exercise2();
    exercise3();
    exercise4();
    // exercise5();
    exercise6();
    exercise7();
    exercise8();
    exercise9();
    exercise10();

}
