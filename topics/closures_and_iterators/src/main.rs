

fn exercises(){
    // 1. Write a closure that takes a `String` and returns its length. Use it on multiple strings.
    let string_size = |s: String|s.len();
    println!("{}",string_size(String::from("Hi I am adarsh")));
    // 2. Define a closure that mutably captures and modifies a counter from the environment. 
    // Call it multiple times.
    let mut counter = 1;
    let mut inc_count = ||{
        counter += 1;
        println!("inside closure: {}",counter);
    };
    for _ in 0..3{
        inc_count();
    }
    println!("{}",counter);
    // 3. Store a closure in a variable, pass it to a function as an argument, 
    // and invoke it inside that function.
    let expo = |x:i32, y:u32| x.pow(y);
    println!("{}",expo(3,2));
    // 4. Create a vector of integers. Use `.iter()` to print each value, then use `.into_iter()` 
    // and compare the behavior.
    let vec = vec![2,3,4,5,6,12];
    for val in vec.iter(){
        println!("{}",val);
    };
    for val in vec.clone().into_iter(){
        println!("{}",val);
    }
    // 5. Chain `.map()` and `.filter()` to extract all odd numbers, double them, 
    // and collect into a new vector.
    let new_vec: Vec<i32> = vec.iter().filter(|x|**x % 2 == 0).map(|x|x * x).collect();
    println!("{:?}",new_vec);
    // 6. Create a closure that returns another closure. 
    // The inner closure should multiply by a captured factor.
    let nested_closure = |x:&str|{

        println!("from nested: {}",x);
        let inner_closure = ||{
            println!("from inner: {}",x);
        };
        inner_closure();
    };
    nested_closure("Yes");
    
    // 7. Define an iterator chain that filters numbers divisible by 3, squares them, 
    // and sums the result.
    let iter_chain = |v:Vec<i32>|{
        let sum_of_div3:i32 = v.iter().filter(|x|**x % 3 == 0).map(|x|x * x).sum();
        println!("Sum of all 3 multiples: {}",sum_of_div3);
    };
    iter_chain(vec![1,2,3,4,5,6,7,8,9,10,11,12,13]);
    // 8. Implement a function that accepts a vector and a closure, 
    // and returns a new vector of transformed results.
    fn vec_closure(v:Vec<i32>, a:i32) -> Vec<i32>{
        v.iter().map(|x|x * a).collect()
    }
    let clo = |x:i32, y:i32, z:i32| x * y + z;
    let returned_vector = vec_closure(vec![1,2,3,4,5], clo(2,4,6));
    println!("{:?}",returned_vector);
    // 9. Build a pipeline using `.iter()`, `.enumerate()`, and `.filter()` 
    // to collect even-indexed elements.
    fn even_vec (v:Vec<i32>) -> Vec<i32>{
        v.iter().enumerate().filter(|(i,_)|i%2==0).map(|(_,value)|*value).collect()
    }
    let vec9 = vec![12,14,21,24];
    let to_print = even_vec(vec9);
    println!("exercise9: {:?}",to_print);
    // 10. Write a program that compares memory use between `.iter()` and `.into_iter()` 
    // on large vectors using `.collect()` and `.count()`.
    let large_vec:Vec<i32> = (0..1_000_000).collect();
    let iter_collect: Vec<_> = large_vec.iter().map(|x|x * 2).collect();
    let into_iter_collect: Vec<_> = large_vec.clone().into_iter().map(|x|x* 2).collect();
    println!("{},{}", iter_collect.len(),size_of_val(&iter_collect));
    println!("{},{}", into_iter_collect.len(),size_of_val(&into_iter_collect));
}
// closures_and_iterators.rs

// 🔸 Closures are anonymous functions that can capture variables from their environment.
// Syntax: |args| -> return_type { body }
// Usually the type and return are inferred.


fn main() {
    closure_examples();
    iterator_examples();

    let result = sum_of_squares_of_even_numbers(vec![1, 2, 3, 4, 5, 6]);
    println!("sum of squares of evens: {}", result);
    println!("*******************");
    exercises();
}



fn closure_examples() {
    // Basic closure that adds one
    let add_one = |x: i32| x + 1;
    println!("add_one(5): {}", add_one(5));

    // Closure capturing variable from outer scope
    let num = 10;
    let add_num = |x: i32| x + num;
    println!("add_num(2): {}", add_num(2));

    // Closures can be stored and reused
    let multiply = |a: i32, b: i32| a * b;
    println!("multiply(4, 5): {}", multiply(4, 5));
}

// 🔸 Iterator basics: `.iter()`, `.into_iter()`, `.map()`, `.filter()`, `.collect()`

fn iterator_examples() {
    let nums = vec![1, 2, 3, 4, 5];

    // .iter() gives references
    for val in nums.iter() {
        println!("ref: {}", val);
    }

    // .into_iter() consumes the vector
    for val in nums.clone().into_iter() {
        println!("owned: {}", val);
    }

    // Use map and collect to transform
    let squared: Vec<i32> = nums.iter().map(|x| x * x).collect();
    println!("squared: {:?}", squared);

    // Filter even numbers
    let evens: Vec<i32> = nums.iter().filter(|x| **x % 2 == 0).cloned().collect();
    println!("evens: {:?}", evens);
}

// 🔸 Combining closures and iterators
fn sum_of_squares_of_even_numbers(v: Vec<i32>) -> i32 {
    v.into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum()
}

