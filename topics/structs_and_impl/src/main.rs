// Topic: Structs and impl

fn main() {
    // Basic struct
    let user1 = User {
        username: String::from("adarsh"),
        email: String::from("adarsh@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username); // adarsh

    // Struct update syntax
    let user2 = User {
        email: String::from("adarsh2@example.com"),
        ..user1 // copies remaining fields from user1
    };

    println!("User2 email: {}", user2.email); // adarsh2@example.com

    // Tuple struct
    let black = Color(0, 0, 0);
    println!("Black RGB: {}, {}, {}", black.0, black.1, black.2); // 0, 0, 0

    // Unit-like struct
    let _marker = Marker;

    // Using associated function (constructor)
    let u = User::new("alex@example.com", "alex");
    println!("New user: {}", u.username); // alex
}

// Named field struct
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// Tuple struct
struct Color(i32, i32, i32);

// Unit-like struct
struct Marker;

// Associated functions & methods with impl block
impl User {
    fn new(email: &str, username: &str) -> User {
        User {
            email: String::from(email),
            username: String::from(username),
            active: true,
            sign_in_count: 1,
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn greet(&self) {
        println!("Hi, I'm {}", self.username);
    }
}


//Solve these again to recall these concept

// 1. Define a `User` struct instance manually and print each field.
// 2. Use struct update syntax to create a second user from the first with one changed field.
// 3. Implement a tuple struct `Point(i32, i32)` and print its coordinates.
// 4. Create and use a unit-like struct `Logger` that does nothing.
// 5. Implement an associated function `User::default()` that returns a default user.
// 6. Implement a method `change_email(&mut self, new_email: &str)` and test it.
// 7. Call the `is_active` method and print the returned value.
// 8. Call the `greet` method and verify the output.
// 9. Refactor `User::new` to take ownership of the strings and call it with `String::from()`.
// 10. Create a function that takes a `User` as parameter and prints all fields using a method inside `impl`.

//SOLUTIONS:-

// Define a User struct instance manually and print each field.
// struct User{
//     name: String,
//     age: i32,
//     is_online: bool,
// }
// fn main(){
//     let user1 = User{
//         name: String::from("Adarsh"),
//         age: 26,
//         is_online: true,
//     };
//     println!("{:?} {:?} {:?}",user1.name,user1.age,user1.is_online);
    
// }
// Use struct update syntax to create a second user from the first with one changed field.
// #[derive(Debug, Clone)]
// struct User{
//     name: String,
//     age: i32,
//     is_online: bool,
// }
// fn main(){
//     let user1 = User{
//         name: String::from("Adarsh"),
//         age: 26,
//         is_online: true,
//     };
//     let user2 = User{
//         is_online: false, 
//         ..user1.clone()
//     };
//     println!("{:?} {:?} {:?}",user1.name,user2.age,user2.is_online);
// }
// Implement a tuple struct Point(i32, i32) and print its coordinates.
// #[derive(Debug)]
// struct Point(i32, i32);
// fn main(){
//     let coor = Point(20,34);
//     println!("Coordinates are: ({:?}, {:?})",coor.0, coor.1);
// }
// Create and use a unit-like struct Logger that does nothing.
// #[warn(unused_variables)]
// struct Logger;
// fn main(){
//     let _do_nothing_logger = Logger;
// }
// Implement an associated function User::default() that returns a default user.
// struct User{
//     name: String,
//     age: i32,
//     is_active: bool
// }
// impl User{
//     fn new(name: String, age: i32) -> User{
//         User{
//             name: name,
//             age: age,
//             is_active: true,
//         }
//     }
// }
// fn main(){
//     let use_me = User::new(String::from("A"), 24);
//     println!("{:?} is {:?} years old and Active: {:?}",use_me.name, use_me.age, use_me.is_active);
// }
// Implement a method change_email(&mut self, new_email: &str) and test it.
// struct User{
//     email: String,
//     age: i32,
// }
// impl User{
//     fn change_email(&mut self, new_email:String){
//         self.email= new_email;
//     }
// }
// fn main(){
//     let mut u1 = User{
//         email: String::from("Noi"),
//         age: 54
//     };
//     println!("{:?}",u1.email);
//     u1.change_email(String::from("hello"));
//     println!("{:?}",u1.email);
// }
// Call the is_active method and print the returned value.
// struct User{
//     is_active: bool,
//     name: String,
// }
// impl User{
//     fn is_active(&self)->bool{
//         self.is_active
//     }
// }
// fn main(){
//     let user = User{
//         is_active: false,
//         name: String::from("hey")
//     };
//     let result = user.is_active();
//     println!("{:?}",result);
//     println!("{:?}",user.name);
// }
// Call the greet method and verify the output.
// struct User{
//     name: String,
    
// }
// impl User{
//     fn new(name: String) -> User{
//         User{
//             name,
//         }
//     }
//     fn greet(&self)->String{
//         self.name.clone()
//     }
// }
// fn main(){
//     let user = User::new(String::from("main"));
//     let person = user.greet();
//     println!("Hey {:?} function",person)
// }
// Refactor User::new to take ownership of the strings and call it with String::from().
//done

// Create a function that takes a User as a parameter and 
// prints all fields using a method inside impl.

// struct User{
//     name: String,
//     age: i32,
//     is_active: bool
// }
// impl User{
//     fn new(name:String, age:i32, is_active:bool)->User{
//         User{
//             name, age, is_active
//         }
//     }
//     fn u_name(&self) -> String{
//         self.name.clone()
//     }
//     fn u_age(&self) -> i32{
//         self.age
//     }
//     fn u_is_active(&self) -> bool{
//         self.is_active
//     }
// }
// fn main(){
//     let user = User::new(String::from("Adarsh"), 27, false);
//     println!("{:?} is {:?} years old. Status: {:?}",user.u_name(),user.u_age(),user.u_is_active());
// }
