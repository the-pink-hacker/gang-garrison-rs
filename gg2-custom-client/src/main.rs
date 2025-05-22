mod asset;
mod camera;
mod config;
mod error;
mod init;
mod networking;
mod player;
mod prelude;
mod render;

use prelude::*;

fn main() -> Result<(), ClientError> {
    init::App::new().start()
}
