use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, name = "horizon", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Initializes the Horizon environment locally")]
    Init,
}
