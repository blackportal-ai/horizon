pub mod commands;
pub mod job_cmds;

use clap::Parser;
use commands::{Cli, Command};

use color_eyre::eyre::Report;
use job_cmds::JobCommand;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Init => {
            JobCommand::init_command().await;
            Ok(())
        }
    }
}
