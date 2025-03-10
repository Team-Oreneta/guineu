use crate::println;
use crate::clear;

pub fn find_command(command: &str) {
    // Split the input: everything before the first space is the command,
    // and anything after (trimmed) is arguments.
    let (rawcommand, args) = match command.find(' ') {
        Some(index) => {
            let cmd = &command[..index];
            let args = command[index..].trim_start();
            (cmd, args)
        }
        None => (command, ""),
    };

    match rawcommand {
        "echo" => println!("{}", args),
        //meta stuff:
        "clear" => clear::clear(),
        "help" => println!("Available commands: echo"),
        _ => println!("Unknown command!"),
    }
}