use std::io::stdin;

use cli::command::Command;

fn main() {
    println!("= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
    println!("PhoneBook v{:} by APEX (c)\n", env!("CARGO_PKG_VERSION"));

    println!("Commands:");
    Command::print_commands();

    let mut buf = String::new();
    let mut is_quit_requested = false;
    while (!is_quit_requested) {
        buf.clear();
        println!("Enter command & arguments:");
        stdin().read_line(&mut buf).expect("failed to read command");

        // @TODO:
    }


    println!("\n= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
}
