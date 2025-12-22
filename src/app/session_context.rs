use tokio::runtime::Runtime;

pub struct SessionStatus {
    pub runtime: Runtime,
    pub should_quit: bool,
}

impl SessionStatus {
    pub fn new() -> rustyline::Result<Self> {
        Ok(Self {
            runtime: Runtime::new()?,
            should_quit: false,
        })
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

}
