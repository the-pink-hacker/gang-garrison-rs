mod error {
    use crate::prelude::*;

    pub type Result<T> = std::result::Result<T, Error>;

    /// All of Gang Garrison's errors
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Winit error: {0}")]
        WinitEventLoop(#[from] winit::error::EventLoopError),
    }
}

mod init {
    use crate::prelude::*;

    #[derive(Default)]
    pub struct App;

    impl App {
        /// Initializes the client and begins the game loop
        pub fn run_client(mut self) -> Result<()> {
            env_logger::init();
            self.init_render()?;

            Ok(())
        }
    }
}

mod prelude {
    pub use crate::error::*;
}

mod render {
    use winit::{
        application::ApplicationHandler,
        event::*,
        event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
        window::WindowId,
    };

    use crate::init::App;
    use crate::prelude::*;

    impl App {
        /// Initializes render loop
        pub fn init_render(&mut self) -> Result<()> {
            let event_loop = EventLoop::new()?;

            // For now, render as fast as possible
            event_loop.set_control_flow(ControlFlow::Poll);

            event_loop.run_app(self)?;

            Ok(())
        }
    }

    impl ApplicationHandler for App {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
        }
    }
}

use prelude::*;

fn main() -> Result<()> {
    init::App::default().run_client()
}
