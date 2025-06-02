// file_io_basics.rs

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
fn main() -> io::Result<()> {
    write_to_file()?;
    append_to_file()?;

    let content = read_from_file()?;
    println!("File content (manual read):\n{}", content);

    let quick = quick_read_all()?;
    println!("File content (quick read):\n{}", quick);

    Ok(())
}

// 1. Create a file and confirm it overwrites any existing content.

// 2. Write multiple lines with `write_all`, then inspect newline handling.
// 3. Append data with `OpenOptions`, verify no overwrite occurs.
// 4. Attempt to append without creating the file first, observe the error.
// 5. Read file with `read_to_string`, validate full content retrieval.
// 6. Manually truncate file, then read again to confirm zero content.
// 7. Use `fs::read_to_string` for a one-liner read, compare output with manual method.
// 8. Check file permission error by making file read-only before writing.
// 9. Combine write, append, and read in sequence, inspect file state after each.
// 10. Simulate a read failure by providing an invalid path, handle the error manually.

fn write_to_file() -> io::Result<()> {
    let mut file = File::create("output.txt")?; // Creates or overwrites
    file.write_all(b"Hello, file world!\n")?;
    Ok(())
}

fn append_to_file() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("output.txt")?;
    file.write_all(b"Appending a new line.\n")?;
    Ok(())
}

fn read_from_file() -> io::Result<String> {
    let mut file = File::open("output.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn quick_read_all() -> io::Result<String> {
    fs::read_to_string("output.txt") // One-liner file read
}


