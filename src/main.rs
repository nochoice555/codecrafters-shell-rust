use env::var;
use std::{
    env,
    fs::metadata,
    io::{self, Write},
    path::Path,
    process::{exit, Command},
    str::FromStr,
};

#[derive(Debug)]
enum Commands {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
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
            "pwd" => Ok(Self::Pwd),
            "cd" => Ok(Self::Cd),
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
            [cmd, ..] => match cmd.parse() {
                Ok(cmd_parsed) => match cmd_parsed {
                    Commands::Exit => exit(commands[1].parse().unwrap_or(0)),
                    Commands::Echo => println!("{}", commands[1..].join(" ")),
                    Commands::Type => find_type(commands[1]),
                    Commands::Pwd => print_cur_dir(),
                    Commands::Cd => cd_dir(&commands[1..].join(" ")),
                },
                Err(_) => exec_path_or_not_found(&cmd, &commands),
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
    input.trim().split(' ').collect()
}

fn print_cur_dir() {
    let pwd = env::current_dir().unwrap();
    println!("{}", pwd.display());
}

fn find_type(command: &str) {
    match command.parse::<Commands>() {
        Ok(_) => println!("{} is a shell builtin", command),
        Err(_) => match var("PATH")
            .unwrap()
            .split(":")
            .map(|path| format!("{}/{}", path, command))
            .find(|path| metadata(path).is_ok())
        {
            Some(path) => println!("{} is {}", command, path),
            _ => println!("{} not found", command),
        },
    }
}

fn exec_path_or_not_found(cmd: &str, commands: &Vec<&str>) {
    let binding = var("PATH").unwrap();
    let paths: Vec<&str> = binding.split(":").collect();
    let cmd_path = Path::new(&paths[0]).join(cmd);

    if cmd_path.exists() {
        Command::new(cmd_path)
            .args(&commands[1..])
            .status()
            .expect("Failed to execute process");
    } else {
        println!("{}: command not found", cmd)
    }
}

fn cd_dir(args: &str) {
    let new_arg = match args {
        "~" => var("HOME").unwrap_or("HOME".to_string()),
        _ => args.to_string(),
    };
    let new_path = Path::new(&new_arg);

    if env::set_current_dir(new_path).is_err() {
        println!("{}: No such file or directory", args)
    }
}
