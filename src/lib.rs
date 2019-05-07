use std::str::FromStr;

pub struct RshLoop;

impl RshLoop {
    pub fn new() -> RshLoop {
        RshLoop
    }

    pub fn run(&self) -> Result<(), failure::Error> {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let args: Vec<String> = input.split_whitespace().map(String::from).collect();

            Self::exec(args)?
        }
    }

    fn exec(args: Vec<String>) -> Result<(), failure::Error> {
        if args.is_empty() {
            failure::bail!("Input command!");
        }
    }
}

#[derive(Debug)]
enum Commands {
    Cd,
    Help,
    Exit,
    Exec,
}

impl FromStr for Commands {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Commands, Self::Err> {
        match s {
            "cd" => Ok(Commands::Cd),
            "help" => Ok(Commands::Help),
            "exit" => Ok(Commands::Exit),
            _ => Ok(Commands::Exec),
        }
    }
}
