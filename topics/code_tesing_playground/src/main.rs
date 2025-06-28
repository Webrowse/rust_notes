// 3. Positional Variadic Argument
// Run:
// cargo run -- file1.txt file2.txt  

// Confirm files captured.
// Then run with zero files. 
// Confirm it doesn’t panic. 
// Change num_args(1..) to 1..=3. Test limits.

use clap::{Arg, Command};

fn main(){
    let matches = Command::new("cli-tool")
        .version("1.0")
        .author("Adarsh")
        .arg(
            Arg::new("files")
                .help("list of files to process")
                .num_args(1..)
                .value_parser(clap::value_parser!(String))   
        ).get_matches();

    if let Some(files_iter) = matches.get_many::<String>("files"){
        let files: Vec<&String> = files_iter.collect();
        println!("Captured file (count {}) : {:#?}", files.len(), files);
    }
    else{
        println!("No files captured");
    }
}