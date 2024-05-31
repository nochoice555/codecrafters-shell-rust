use std::str::FromStr;
use std::{
    io::{self, Write},
    process::exit,
};

#[derive(Debug)]
enum Commands {
    Exit,
    Echo,
    Type,
}

#[derive(Debug)]
struct CommandsError;

impl FromStr for Commands {
    type Err = CommandsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Self::Exit),
            "echo" => Ok(Self::Echo),
            "type" => Ok(Self::Type),
            _ => Err(CommandsError),
        }
    }
}

fn main() {
    loop {
        print!("$ ");

        let mut input = String::new();
        read_input_to_buff(&mut input);

        let commands = parse_command(&mut input);

        match commands[..] {
            [cmd] => println!("{}: command not found", cmd),
            [cmd, ..] => match cmd.parse() {
                Ok(cmd_parsed) => match cmd_parsed {
                    Commands::Exit => exit(commands[1].parse().unwrap_or(0)),
                    Commands::Echo => println!("{}", commands[1..].join(" ")),
                    Commands::Type => match commands[1].parse::<Commands>() {
                        Ok(_) => println!("{} is a shell builtin", commands[1]),
                        Err(_) => println!("{} not found", commands[1]),
                    },
                },
                Err(_) => println!("{}: command not found", cmd),
            },
            _ => println!("command match error"),
        }
    }
}

fn read_input_to_buff(input: &mut String) {
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    stdin.read_line(input).unwrap();
}

fn parse_command(input: &str) -> Vec<&str> {
    input.trim().to_lowercase().leak().split(' ').collect()
}
