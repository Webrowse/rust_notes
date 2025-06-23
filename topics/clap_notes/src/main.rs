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
