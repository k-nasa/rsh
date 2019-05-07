pub struct RshLoop;

impl RshLoop {
    pub fn new() -> RshLoop {
        RshLoop
    }

    pub fn run(&self) -> Result<(), failure::Error> {
        loop {}
    }
}
