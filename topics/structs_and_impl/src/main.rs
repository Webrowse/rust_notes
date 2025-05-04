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
