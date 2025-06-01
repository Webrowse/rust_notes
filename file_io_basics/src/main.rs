// file_io_basics.rs

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

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

fn main() -> io::Result<()> {
    write_to_file()?;
    append_to_file()?;

    let content = read_from_file()?;
    println!("File content (manual read):\n{}", content);

    let quick = quick_read_all()?;
    println!("File content (quick read):\n{}", quick);

    Ok(())
}
