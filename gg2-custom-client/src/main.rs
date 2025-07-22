mod asset;
mod camera;
mod config;
mod error;
mod game;
mod init;
mod input;
mod map;
mod networking;
mod player;
mod prelude;
mod render;
mod sync;
mod world;

use prelude::*;

fn main() -> Result<(), ClientError> {
    init::App::new().start()
}
