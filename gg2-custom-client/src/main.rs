mod asset;
mod camera;
mod config;
mod error;
mod init;
mod map;
mod networking;
mod player;
mod prelude;
mod render;
mod sync;

use prelude::*;

fn main() -> Result<(), ClientError> {
    init::App::new().start()
}
