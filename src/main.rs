use std::{io::Error, str::FromStr};
use std::{
    io::{self, Write},
    process::exit,
};

enum Commands {
    Exit,
    NotFound,
}

impl FromStr for Commands {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Self::Exit),
            _ => Ok(Self::NotFound),
        }
    }
}

fn main() {
    print!("$ ");

    loop {
        let mut input = String::new();
        read_input_to_buff(&mut input);

        let [command, code] = parse_command(&mut input)[..] else {return;};
        
        match command.parse() {
            Ok(cmd) => match cmd {
                Commands::Exit => exit(code.parse().unwrap_or(0)),
                Commands::NotFound => println!("{}: command not found", input),
            },
            Err(e) => println!("{}", e),
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
