// fn main() {
//     let mut s = String::from("hello");
    
    
//     // push and push_str
//     s.push(' ');
//     s.push_str("world");
//     println!("{}", s); // "hello world"
    
//     // len and is_empty
//     println!("len: {}", s.len()); // 11
//     println!("empty? {}", s.is_empty()); // false
    
//     // contains and replace
//     println!("contains 'lo'? {}", s.contains("lo")); // true
//     let replaced = s.replace("world", "rust");
//     println!("replaced: {}", replaced); // "hello rust"
    
//     // split and collect
//     for word in s.split_whitespace() {
//         println!("word: {}", word);
//     }
    
//     // trim and to_uppercase
//     let messy = "  rust  ".to_string();
//     println!("trimmed: '{}'", messy.trim()); // 'rust'
//     println!("upper: {}", messy.to_uppercase()); // "  RUST  "
    
//     // indexing not allowed
//     // println!("{}", s[0]); // ❌ won't compile
    
//     }
    
    // Exercise
fn main(){

    // 1. Create a `String`, push multiple characters and words, print final value.
    let mut str1 = String::from("hello");
    str1.push(' ');
    str1.push(':');
    str1.push_str(" yeah");
    println!("1. str1: {}",str1);

    // 2. Check `len` and `is_empty` before and after modifying the string.
    let mut str2 = String::new();
    println!("2. len before: {}",str2.len());
    println!("   is_empty before: {}",str2.is_empty());
    str2.push_str("Apple");
    println!("   len after: {}",str2.len());
    println!("   is_empty after: {}",str2.is_empty());
    // 3. Use `contains` with various substrings to test pattern matching.
    println!("3. str2 contains 'ban'?: {}",str2.contains("ban"));
    println!("   str2 conatins 'pp'?: {}",str2.contains("pp"));
    // 4. Replace different words inside the string and print each result.
    let str4 = String::from("hello rust");
    println!("4. str4 before replace(): {}",str4);
    let replaced = str4.replace("rust","crusty");
    println!("   str4 after replace(): {}",replaced);
    // 5. Split a sentence into words using `split_whitespace`, print each.
    let str5 = String::from("hello hi i'm learning rust");
    println!("5. split_whitespace: ");
    for str in str5.split_whitespace(){
        println!("    {}",str);
    }
    // 6. Trim a string with mixed whitespace, confirm leading/trailing removal.
    let str6 = String::from("   hi  yess    ");
    println!("6. untrimmed: {}",str6);
    println!("   trimmed: {}",str6.trim());
    // 7. Convert a lowercase string with padding to uppercase and print it.
    println!("7. UPPERCASING str6: {}",str6.to_uppercase().trim());
    // 8. Attempt direct indexing like `s[0]`, observe and explain compiler error.
    // println!("8. direct indexing: {}", str6.trim().[0]);
    // 9. Chain methods: trim → to uppercase → replace → collect into `Vec<_>`.
    let s9 = String::from("       lowercase text    ");
    let uped_s9: String = s9.trim().replace("lowercase","uppercase").to_uppercase();
    let vec_s9: Vec<_> = uped_s9.split_whitespace().collect();
    println!("{:?}",vec_s9);
    // 10. Compare `String` and `&str` methods—build a table of method availability.
    let cs = "  hello  rust  world";
    let ct = cs.trim();
    println!("{}",ct); //Hence, trim() works both on &str and String.
     //Hence, push() do not works both on &str but works on String.
    println!("{}",cs.len()); //len() works on both &str and String
    println!("{}",cs.is_empty()); //is_empty() works on both &str and String

    println!("{}",cs.contains("el")); //contains() works on both &str and String
    println!("{}",cs.replace("el","le")); // replace() works on both &str and String

    for s in cs.split_whitespace(){
        println!("{}",s);
    } // spilt_whitespace works on both &str and String

    //indexing is not allowed in both &str and string.
}