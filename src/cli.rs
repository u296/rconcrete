use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
pub enum Command {
    /// Connect to a known server
    Connect(crate::commands::connect::ConnectArgs),
    /// Manage known servers
    Server {
        #[command(subcommand)]
        command: crate::commands::server::ServerCommand,
    },
}
