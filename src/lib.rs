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

        match Commands::from_str(&args[0])? {
            Commands::Cd => Self::exec_cd(&args[1]), // TODO error handle
            Commands::Exec => Self::exec_exec(),
            Commands::Exit => Self::exec_exit(),
            Commands::Help => Self::exec_help(),
        }
    }

    fn exec_cd(dir_path: &str) -> Result<(), failure::Error> {}

    fn exec_exec() -> Result<(), failure::Error> {
        Ok(())
    }
    fn exec_exit() -> Result<(), failure::Error> {
        Ok(())
    }
    fn exec_help() -> Result<(), failure::Error> {
        Ok(())
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
