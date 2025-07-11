// Run: cargo add serde serde_json --features derive
use serde::{Deserialize, Serialize};
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

fn default_threads() -> usize {
    4
}

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
    let user = User {
        id: 1,
        username: "adarsh".to_string(),
        active: true,
    };
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
    let renamed = RenamedFields {
        id: 10,
        username: "foo".into(),
    };
    println!(
        "4. Renamed field: {}",
        serde_json::to_string(&renamed).unwrap()
    );

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
    println!(
        "8. Flatten: {}",
        serde_json::to_string_pretty(&doc).unwrap()
    );

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

// Exercise 5: Flatten Composition

// use serde::{Deserialize, Serialize};
// use serde_json;

// #[derive(Serialize, Deserialize, Debug)]
// struct Inner{
//     a: i32,
//     b: i32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Outer{
//     id: u32,
//     #[serde(flatten)]
//     data: Inner,
// }

// fn main(){
//     let flatten_serde = Outer{
//         id: 42,
//         data: Inner { a: 69, b: 420 }
//     };
//     let serial_flat = serde_json::to_string_pretty(&flatten_serde).unwrap();
//     println!("{}", serial_flat);

//     let deserial_flat: Outer = serde_json::from_str(&serial_flat).unwrap();
//     match deserial_flat{
//         n if n.id>50 => println!("id is greater than 50: {}",n.id),
//         n if n.id<50 => println!("id is less than 50: {}",n.id),
//         _ => println!("No clue"),
//     }
// }

// Exercise 6: Strict Input Enforcement
// Deserialize StrictInput { name: String } with #[serde(deny_unknown_fields)]
// from JSON including extra field.
//     Print the error clearly.
//     Remove extra field. Deserialize successfully.

// use serde::{Deserialize,Serialize};
// use serde_json;

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(deny_unknown_fields)]
// struct StrictInput{
//     name: String,
// }

// fn main(){
//     let deny_unknown = r#"{"name": "Darsh", "age":19}"#;
//     match serde_json::from_str::<StrictInput>(deny_unknown){
//         Ok(allow) => println!("{:?}", allow),
//         Err(e) => eprintln!("Error, due to more field than expected: {}",e),
//     }
//     let allow_known = r#"{"name": "Darsh"}"#;
//     match serde_json::from_str::<StrictInput>(allow_known){
//         Ok(allow) => println!("{:?}", allow),
//         Err(e) => eprintln!("Error, due to more field than expected: {}",e),
//     }
// }

// Exercise 7: Default Injection
// Create a struct with:
//      #[serde(default)] verbose: bool,
//      #[serde(default = "default_port")] port: u16

//  Provide minimal JSON (missing fields).
//  Confirm defaults fill missing data.
//
// use serde::Deserialize;
// use serde_json;

// #[derive(Debug, Deserialize)]
// struct DefaultSerde {
//     #[serde(default)]
//     verbose: bool,

//     #[serde(default = "default_port")]
//     port: u16,
// }

// fn default_port() -> u16 { 3 }

// fn main() {
//     let default_data = r#"{}"#;
//     let deserialed: DefaultSerde = serde_json::from_str(default_data).unwrap();
//     println!("{:?}", deserialed);
// }

// Exercise 8: Tagged Enum Dispatcher
// Deserialize command pattern:
//  {"type": "Move", "x": 1, "y": 2}
//  {"type": "Stop"}
// From:
//  enum Command { Move { x: i32, y: i32 }, Stop }
// Deserialize and pattern match.

// use serde::Deserialize;
// use serde_json;

// #[derive(Debug, Deserialize)]
// #[serde(tag = "type")]
// enum Command{
//     Move{
//         x: i32,
//         y: i32,
//     },
//     Stop,
// }
//  fn process_print(json_str: &str){
//     match serde_json::from_str::<Command>(json_str){
//         Ok(cmd) => {
//             match cmd{
//                 Command::Move { x, y } => {
//                     println!("Move Command: Moving to {},{}", x, y);
//                 }
//                 Command::Stop => {
//                     println!("Stop Command, do not move");
//                 }
//             }
//         },
//         Err(e) => eprintln!("Error deserialising: {}",e),
//     }
//  }

// fn main(){
//     let enum_data = r#"{"type": "Move", "x": 1, "y": 2}"#;
//     process_print(enum_data);

//     let enum_data2 = r#"{"type": "Stop"}"#;
//     process_print(enum_data2);
// }

// Exercise 9: Deserialize Arbitrary Map
// From JSON: {"x": 10, "y": 20}
//     Deserialize to HashMap<String, i32>
//     Iterate and print key-values
//     Re-serialize back to JSON

// use std::collections::HashMap;
// use serde_json;

// fn main(){
//     let raw_hashmap = r#"{"x": 10, "y": 20}"#;
//     let hashed: HashMap<String, i32> = serde_json::from_str(raw_hashmap).unwrap();
//     println!("{:?}", hashed);

//     for (i, j) in hashed.iter(){
//         println!("Key: {:?}, Value: {:?}",i, j);
//     }

//     let serialise_json = serde_json::to_string(&hashed).unwrap();
//     println!("{}", serialise_json);
// }

// Exercise 10: Error Handling Practice
// Wrap all deserialization attempts in a match or Result.
//     Print errors without crashing
//     Try invalid JSON strings: missing fields, extra fields, wrong types.

// use serde::{Deserialize, Serialize};
// use serde_json;

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(deny_unknown_fields)]
// struct Testing{
//     name: String,
//     age: u32,
// }

// fn deserial_error_free(json_str: &str){
//     match serde_json::from_str::<Testing>(json_str){
//         Ok(res) => {
//             println!("{:?}", res);
//         },
//         Err(e) => {
//             eprintln!("The error while Deserialisation: {}",e);
//         }
//     }
// }
// fn main(){
//     let sample1 = Testing{ name: "Romy".into(), age: 19};
//     let test1  = serde_json::to_string(&sample1).unwrap();
//     println!("{}",test1);
//     println!("Test 1: ");
//     deserial_error_free(test1.as_str());

//     println!("Test 2: ");
//     let test2 = r#"{"name": "romy"}"#;
//     deserial_error_free(test2);

//     println!("Test 3: ");
//     let test3 = r#"{"name": "romy", "age": 19, "email": "a@we.com"}"#;
//     deserial_error_free(test3);

//     println!("Test 4: ");
//     let test4 = r#"{"name": "romy", "age": true,}"#;
//     deserial_error_free(test4);
// }
