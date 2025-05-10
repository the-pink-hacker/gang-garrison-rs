mod asset;
mod camera;
mod config;
mod error;
mod init;
mod networking;
mod prelude;
mod render;

use prelude::*;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    init::App::new().start().await
}
