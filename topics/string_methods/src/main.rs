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

// 1. Create a `String`, push multiple characters and words, print final value.
// 2. Check `len` and `is_empty` before and after modifying the string.
// 3. Use `contains` with various substrings to test pattern matching.
// 4. Replace different words inside the string and print each result.
// 5. Split a sentence into words using `split_whitespace`, print each.
// 6. Trim a string with mixed whitespace, confirm leading/trailing removal.
// 7. Convert a lowercase string with padding to uppercase and print it.
// 8. Attempt direct indexing like `s[0]`, observe and explain compiler error.
// 9. Chain methods: trim → to\_uppercase → replace → collect into `Vec<_>`.
// 10. Compare `String` and `&str` methods—build a table of method availability.
