use clap::Args;

#[derive(Debug, Args)]
pub struct CommonCliJoinServer {
    /// The server the client will attempt to connect to
    /// Defaults to the client config's default server url
    #[arg(long)]
    pub server_url: Option<String>,
}
