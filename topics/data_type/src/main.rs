// Topic: Data Types

fn main(){
    let a = 10; //numbers are i32 by default
    println!("a = {}", a); //or println!("a = {a}");

    //Float (f64 by default)
    let b = 3.14;
    println!("{b}");

    //Boolean
    let _is_active = true; // start the variable with an underscore, if we aren't going to use it

    //Character(single quotes)
    let _letter = 'R';

    //Tuple (group of different values of different data types)
    let tup = (1, 2.0, 'a', "hello");
    let (w,x,y,z) = tup; //destructuring
    println!("w = {}, x = {}, y = {}, z = {}",w,x,y,z);

    //accessing tuples by index
    println!("first item ={}",tup.0);

    //Array (fixed size, same type)
    let arr = [1,2,3,4]; // [data type; length]
    println!("arr[0] = {}",arr[0]);

    let same = [33;4];
    println!("the array is {:?}",same); //the {} in print! uses Display trait,
    //but array do not implement Display, it implement Debug, 
    // quick fixes {:?}, {:#?}
    //
}