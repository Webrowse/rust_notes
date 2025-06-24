// 1. Arg Count Behavior
// Run with:cargo run -- -vvv  

// Confirm output: Verbosity level: 3
// Then remove ArgAction::Count, observe behavior. 
// Understand count vs boolean action.

use clap::{Arg, Command};

fn main(){
    let matches = Command::new("cli-tool")
    .author("Romy")
    .arg(
        Arg::new("verbose")
            .short('v')
    )
    .get_matches();

    let level = matches.get_count("verbose");
    println!("Verbosity level: {}", level);
    

    if matches.get_flag("verbose"){
        println!("Verbose flag is present");
    }
}