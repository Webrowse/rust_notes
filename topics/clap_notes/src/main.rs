// main.rs

use clap::{Arg, ArgAction, Command, ArgMatches};

fn main() {
    let matches = Command::new("cli-tool")
        .version("1.0")
        .author("Your Name")
        .about("Demonstrates clap usage")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Increases verbosity")
                .action(ArgAction::Count),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Sets a name")
                .num_args(1),
        )
        .arg(
            Arg::new("files")
                .help("Input files")
                .num_args(1..)
                .value_hint(clap::ValueHint::FilePath),
        )
        .subcommand(
            Command::new("add")
                .about("Adds a value")
                .arg(
                    Arg::new("value")
                        .help("Value to add")
                        .required(true)
                        .value_parser(clap::value_parser!(i32)),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Removes a value")
                .arg(
                    Arg::new("value")
                        .help("Value to remove")
                        .required(true)
                        .value_parser(clap::value_parser!(i32)),
                ),
        )
        .get_matches();

    handle_args(&matches);
}

fn handle_args(matches: &ArgMatches) {
    // Handle top-level args
    if let Some(level) = matches.get_count("verbose") {
        println!("Verbosity level: {}", level);
    }

    if let Some(name) = matches.get_one::<String>("name") {
        println!("Name: {}", name);
    }

    if let Some(files) = matches.get_many::<String>("files") {
        println!("Files:");
        for file in files {
            println!(" - {}", file);
        }
    }

    // Handle subcommands
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let val = sub_m.get_one::<i32>("value").unwrap();
            println!("Adding: {}", val);
        }
        Some(("remove", sub_m)) => {
            let val = sub_m.get_one::<i32>("value").unwrap();
            println!("Removing: {}", val);
        }
        _ => {}
    }
}


//Exercises:

// 1. Arg Count Behavior
// Run with:cargo run -- -vvv  

// Confirm output: Verbosity level: 3
// Then remove ArgAction::Count, observe behavior. 
// Understand count vs boolean action.

// use clap::{Arg, ArgAction, Command};

// fn main(){
//     let matches = Command::new("cli-tool")
//     .author("Romy")
//     .arg(
//         Arg::new("verbose")
//             .short('v')
//             .action(ArgAction::Count)
//     )
//     .get_matches();

//     let level = matches.get_count("verbose");
//     println!("Verbosity level: {}", level);
// }

// 2. Single Arg Extraction
// Run:

// cargo run -- --name adarsh

// Confirm Name: adarsh prints.
// Then remove .num_args(1), run again, and analyze result.

// use clap::{Arg, ArgAction, Command};

// fn main() {
//     let matches = Command::new("cli-tool")
//         .arg(
//             Arg::new("name")
//                 .long("name")
//                 .action(ArgAction::Append)
//                 .value_parser(clap::value_parser!(String)),
//                 // .num_args(1),
//         )
//         .get_matches();

//     if let Some(name) = matches.get_many::<String>("name") {
//         for i in name{
//             println!("Get many: {}", i);
//         }
//     }
// }

