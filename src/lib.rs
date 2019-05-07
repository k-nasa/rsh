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
        Ok(())
    }
}
