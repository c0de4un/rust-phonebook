use std::io::stdin;

use cli::{command::{Command, CommandFactory}, e_cli_command_types::ECLICommandTypes};

fn main() {
    println!("= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
    println!("PhoneBook v{:} by APEX (c)\n", env!("CARGO_PKG_VERSION"));

    println!("Commands:");
    CommandFactory::create("HELP")
        .unwrap()
        .run();

    let mut buf = String::new();
    let mut is_quit_requested = false;
    while !is_quit_requested {
        buf.clear();
        println!("Enter command & arguments:");
        stdin().read_line(&mut buf).expect("failed to read command");
        buf = buf.trim().to_lowercase();

        // @TODO:
    }


    println!("\n= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
}
