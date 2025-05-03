use clap::{Parser, Subcommand};
use gg2_custom_common::init::cli::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct ClientCliArguments {
    #[command(subcommand)]
    pub command: Option<ClientCliSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum ClientCliSubcommand {
    JoinServer(CommonCliJoinServer),
}

pub fn init() -> ClientCliArguments {
    ClientCliArguments::parse()
}
