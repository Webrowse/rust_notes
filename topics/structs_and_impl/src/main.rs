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
