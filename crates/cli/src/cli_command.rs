use mustash_core::commands::Command;

use clap::{Parser, Subcommand};
use cli_macros::cli_command;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand
}

#[derive(Subcommand)]
#[cli_command(name="Test", args="TestCommand")]
pub enum CliCommand {
    Test {
        #[arg()]
        arg: String,
    },
}

#[cfg(not(feature = "dispatcher"))]
impl From<CliCommand> for Command {
    fn from(_cmd: CliCommand) -> Self {
        unimplemented!()
    }
}

#[cfg(feature = "dispatcher")]
use cli_macros::cli_dispatcher;

#[cfg(feature = "dispatcher")]
#[cli_dispatcher]
impl From<CliCommands> for Command {
}
