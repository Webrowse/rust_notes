// Topic: References and Slices

fn main() {
    // --- String Slices ---

    let s = String::from("hello world");

    let hello = &s[0..5]; // slice from index 0 to 4
    let world = &s[6..11]; // slice from index 6 to 10

    println!("First: {}, Second: {}", hello, world); // First: hello, Second: world

    // Slice with full range
    let full = &s[..];
    println!("Full slice: {}", full); // hello world

    // --- Function using string slice ---
    let msg = String::from("greetings from rust");
    let first = first_word(&msg);

    println!("First word: {}", first); // greetings

    // --- Array Slices ---
    let nums = [10, 20, 30, 40, 50];
    let part = &nums[1..4]; // slice of [20, 30, 40]

    println!("Array slice: {:?}", part); // [20, 30, 40]
}

// Returns the first word of a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // return full string if no space
}
