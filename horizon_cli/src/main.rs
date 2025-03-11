pub mod commands;

use clap::Parser;
use commands::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Init => {
            // TODO: How do we want to manage local configuration? Initialization of a toml with
            // mappings? Confy crate? Other ideas/suggestions?
            todo!()
        }
    }
}
