use std::{io::Error, str::FromStr};
use std::{
    io::{self, Write},
    process::exit,
};

#[derive(Debug)]
enum Commands {
    Exit,
    Echo,
    NotFound,
}

impl FromStr for Commands {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Self::Exit),
            "echo" => Ok(Self::Echo),
            _ => Ok(Self::NotFound),
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
                    Commands::NotFound => println!("{}: command not found", cmd),
                }
                Err(e) => println!("error - {}", e),
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
