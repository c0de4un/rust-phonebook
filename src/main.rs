use std::io::stdin;

use cli::command::Command;

fn main() {
    println!("= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
    println!("PhoneBook v{:} by APEX (c)\n", env!("CARGO_PKG_VERSION"));

    println!("Commands:");
    Command::print_commands();

    let mut buf = String::new();
    println!("Enter command & arguments:");
    stdin().read_line(&mut buf);


    println!("\n= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
}
