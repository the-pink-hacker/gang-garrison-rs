use crate::prelude::*;

#[derive(Default)]
pub struct App {
    pub render_state: Option<crate::render::State>,
}

impl App {
    /// Initializes the client and begins the game loop
    pub fn run_client(mut self) -> Result<()> {
        env_logger::init();
        self.init_render()?;

        Ok(())
    }
}
