use crate::println;

pub fn find_command(command: &str) {
    println!("[DEBUG] Pathtracer is parsing your command...");

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
        "help" => println!("Available commands: echo"),
        _ => println!("Unknown command!"),
    }
}