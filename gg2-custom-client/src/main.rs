mod error;
mod init;
mod prelude;
mod render;

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    init::App::default().run_client()
}
