mod camera;
mod error;
mod init;
mod networking;
mod prelude;
mod render;

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    init::App::new().start().await
}
