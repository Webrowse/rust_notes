// Run: cargo add serde serde_json --features derive
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    description: Option<String>, // Optional field
}

#[derive(Serialize, Deserialize, Debug)]
struct RenamedFields {
    #[serde(rename = "user_id")]
    id: u32,
    #[serde(rename = "userName")]
    username: String,
}

#[derive(Serialize, Debug)]
struct SkipExample {
    key: String,
    #[serde(skip_serializing)]
    secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Active,
    Inactive,
    #[serde(rename = "sleeping")]
    Idle,
}

#[derive(Serialize, Deserialize, Debug)]
struct WithEnum {
    name: String,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct StrictConfig {
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Defaults {
    name: String,
    #[serde(default)]
    threads: usize,
    #[serde(default = "default_threads")]
    retries: usize,
}

fn default_threads() -> usize { 4 }

#[derive(Serialize, Debug)]
struct Meta {
    created_by: String,
    timestamp: u64,
}

#[derive(Serialize, Debug)]
struct WithFlatten {
    title: String,
    #[serde(flatten)]
    meta: Meta,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum Command {
    Add { value: i32 },
    Remove { id: String },
}

fn main() {
    // 1. Basic Struct Serialize
    let user = User { id: 1, username: "adarsh".to_string(), active: true };
    let json = serde_json::to_string(&user).unwrap();
    println!("\n1. Serialize User: {}", json);

    // 2. Deserialize Struct
    let data = r#"{"id":2,"username":"singh","active":false}"#;
    let user2: User = serde_json::from_str(data).unwrap();
    println!("2. Deserialize User: {:?}", user2);

    // 3. Optional Fields
    let conf_json = r#"{"name": "myapp"}"#;
    let config: Config = serde_json::from_str(conf_json).unwrap();
    println!("3. Optional field: {:?}", config);

    // 4. Renamed Fields
    let renamed = RenamedFields { id: 10, username: "foo".into() };
    println!("4. Renamed field: {}", serde_json::to_string(&renamed).unwrap());

    // 5. Skip Fields
    let skip = SkipExample {
        key: "public_key".into(),
        secret: "super_secret".into(),
        description: None,
    };
    println!("5. Skip example: {}", serde_json::to_string(&skip).unwrap());

    // 6. Enums
    let enum_json = r#"{"name":"adarsh","status":"sleeping"}"#;
    let with_enum: WithEnum = serde_json::from_str(enum_json).unwrap();
    println!("6. Enum field: {:?}", with_enum);

    // 7. HashMap Deserialize
    let map_json = r#"{"name":"adarsh","role":"dev"}"#;
    let map: HashMap<String, String> = serde_json::from_str(map_json).unwrap();
    println!("7. HashMap: {:?}", map);

    // 8. Flatten
    let doc = WithFlatten {
        title: "MyDoc".into(),
        meta: Meta {
            created_by: "admin".into(),
            timestamp: 123456,
        },
    };
    println!("8. Flatten: {}", serde_json::to_string_pretty(&doc).unwrap());

    // 9. Deny Unknown Fields
    let good = r#"{"version": "1.0"}"#;
    let strict: StrictConfig = serde_json::from_str(good).unwrap();
    println!("9. Deny unknown: {:?}", strict);

    // 10. Defaults
    let d_json = r#"{"name":"x"}"#;
    let d: Defaults = serde_json::from_str(d_json).unwrap();
    println!("10. Defaults: {:?}", d);

    // 11. Tagged Enums
    let add_cmd = r#"{"type": "Add", "value": 100}"#;
    let cmd: Command = serde_json::from_str(add_cmd).unwrap();
    println!("11. Tagged Enum: {:?}", cmd);
}


// exercises

// Exercise 1: Custom Struct Roundtrip
// Create a struct Book with fields: title: String, pages: u32, genre: Option<String>.
//     Serialize it to JSON.
//     Deserialize the JSON back.
//     Confirm original and deserialized structs are equal.

// use serde::{self, Deserialize, Serialize};
// use serde_json;
// #[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
// struct Book{
//     title: String,
//     pages: u32,
//     genre: Option<String>,
// }

// fn main(){
//     let book = Book{
//         title: String::from("Rust Book"),
//         pages: 232,
//         genre: Some("Horror".to_string()),
//     };
//     let json = serde_json::to_string(&book).unwrap();
//     println!("\nSerialized: {}", json);
    
//     let json_de: Book = serde_json::from_str(&json).unwrap();
//     println!("\nDESerialized: {:?}", json_de);
//     println!("equating book and deserialized: {:?}",assert_eq!(book,json_de));
// }

// Exercise 2: Field Renaming and Validation
// Make a struct Person with #[serde(rename = "firstName")] name: String.
//     Serialize an instance.
//     Manually write JSON with incorrect key (e.g., name instead of firstName).
//     Attempt deserialization. Catch and print the error.


// #[derive(Serialize, Deserialize, Debug)]
// struct Renaming{
//     #[serde(rename = "firstName")]
//     name: String,
// }

// fn main(){
//     let rename = Renaming{
//         name: "adarsh".to_string(),
//     };
//     let json = serde_json::to_string(&rename).unwrap();
//     println!("{}",json);

//     let de_json = r#"{"name": "adarsh"}"#;
//     println!("{}", de_json);

//     match serde_json::from_str::<Renaming>(de_json){
//         Ok(n) => println!("Successgully: {:?}",n),
//         Err(e) => {
//             eprintln!("error: {}",e);
//         }
//     };
// }

// Exercise 3: Conditional Serialization
// Create a struct LoginAttempt with user: String, token: Option<String>, 
// #[serde(skip_serializing)] password: String.
//     Serialize with token = None.
//     Confirm which fields are present in JSON.

// use serde::{Deserialize, Serialize};
// use serde_json;

// #[derive(Serialize, Deserialize, Debug)]
// struct LoginAttempt{
//     user: String,
//     token: Option<String>,
//     #[serde(skip_serializing)]
//     password: String
// }

// fn main(){
//     let login_attempt = LoginAttempt{
//         user: "Hacker".to_string(),
//         token: None,
//         password: "pass for hacker".to_string(),
//     };

//     match serde_json::to_string(&login_attempt) {
//         Ok(n) => println!("Serialization data: {}",n),
//         Err(e) => eprintln!("Error occur: {:?}",e),
//     }
// }

// Exercise 4: Enum Deserialization and Matching
// Define enum Mode { Read, Write, #[serde(rename = "readonly")] Locked }.
//     Deserialize all three variants from JSON.
//     Match and print behavior per variant.

// use serde::{Deserialize, Serialize};
// use serde_json;

// #[derive(Serialize, Deserialize, Debug)]
// enum Mode{
//     Read,
//     Write,
//     #[serde(rename = "readonly")]
//     Locked
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct WithEnumMode{
//     name: String,
//     mode: Mode,
// }

// fn main(){
//     let json_locked = r#"{"name":"readonly enum", "mode": "readonly"}"#;
//     let json_write = r#"{"name":"write enum", "mode": "Write"}"#;
//     let json_read = r#"{"name":"read enum", "mode": "Read"}"#;
    
//     let read_mode: WithEnumMode = serde_json::from_str(json_read)
//         .expect("deserialize error in read mode");
//     let write_mode: WithEnumMode = serde_json::from_str(json_write)
//     .expect("Deserialize error in write mode");
//     let locked_mode: WithEnumMode = serde_json::from_str(json_locked)
//     .expect("Deserialize error in locked/readonly mode");
    
//     print_mode(&read_mode.mode, "read Mode");
//     print_mode(&write_mode.mode, "write Mode");
//     print_mode(&locked_mode.mode, "Locked Mode");

// }

// fn print_mode(mode: &Mode, struct_name: &str){
//     println!("\nbehavior for {}",struct_name);
//     match mode{
//         Mode::Read => println!("Mode is Read: Data can be viewed"),
//         Mode::Write => println!("Mode is Write: Data can be modified"),
//         Mode::Locked => println!("Mode is Locked Data is read-only"),
//     }
// }