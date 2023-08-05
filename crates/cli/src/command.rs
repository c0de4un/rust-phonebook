// commands.rs
use crate::e_cli_command_types::ECLICommandTypes;

pub trait Command {
    fn run(&self);
}

pub struct HelpCommand;

impl Command for HelpCommand {
    fn run(&self) {
        println!("help");
        println!("create");
        println!("read");
        println!("update");
        println!("delete");
    }
}

pub struct CreateCommand {
    args: Vec<String>,
}

impl Command for CreateCommand {
    fn run(&self) {
        // @TODO: CreateCommand::run()

        for i in 0..self.args.len() {
            let arg = &self.args[i];

            // match arg {
            //     "name" =>
            // }
        }
    }
}

pub struct CommandFactory;

impl CommandFactory {
    pub fn create(s: &str) -> Option<Box<dyn Command>> {
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

        match cmd_type {
            ECLICommandTypes::CREATE => Some(Box::new(CreateCommand{
                args,
            })),
            ECLICommandTypes::READ   => None,
            ECLICommandTypes::UPDATE => None,
            ECLICommandTypes::DELETE => None,
            ECLICommandTypes::HELP   => Some(Box::new(HelpCommand)),
            _                        => Some(Box::new(HelpCommand)),
        }
    }

    pub fn print_commands() {
        let cmd = HelpCommand;
        cmd.run();
    }
}
