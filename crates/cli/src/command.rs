// commands.rs
use crate::e_cli_command_types::ECLICommandTypes;

pub struct Command {
    key: ECLICommandTypes,
    args: Vec<String>,
}

impl Command {
    pub fn from_input(s: &str) -> Option<Command> {
        let v: Vec<&str> = s.split(' ').collect();
        if v.is_empty() {
            return None;
        }

        let val = v.get(0);
        if val == None {
            return None;
        }
        let key = val.expect("");

        let cmd_type = ECLICommandTypes::from_string(key);
        let args: Vec<String> = Vec::with_capacity(v.len() - 1);

        Some(Command {
            key: cmd_type,
            args,
        })
    }

    pub fn print_commands() {
        println!("help");
        println!("create");
        println!("read");
        println!("update");
        println!("delete");
    }
}