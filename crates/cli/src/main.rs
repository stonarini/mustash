mod cli_command;
use cli_command::Cli;

use mustash_core::{commands::Command, dispatcher::Dispatcher};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let cmd: Command = cli.command.into();

    let mut dispatcher = Dispatcher::new();
    let resp = dispatcher.dispatch(cmd);

    println!("{}", resp);
}

#[cfg(test)]
mod tests {
    use mustash_core::commands::Response;

    use super::*;

    #[test]
    fn cli_test_command() {
        let args = (["cli", "test", "hello"]).iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let cli = Cli::try_parse_from(args).expect("Failed to parse");

        let command: Command = cli.command.into();

        let mut dispatcher = Dispatcher::new();
        let response = dispatcher.dispatch(command);

        match response {
            Response::Test(s) => assert_eq!(s, "hello"),
        }
    }
}

