use mustash_macros::command_handler;
use crate::commands::{Response, TestCommand};

use super::Handler;

pub struct TestHandler;
#[command_handler(command = "Test")]
impl Handler<TestCommand, Response> for TestHandler {

    fn handle_command(_cmd: TestCommand) -> Response {
        Response::Test(_cmd.0)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_command() {
        let input = "Test".to_string();
        let command = TestCommand(input.clone());
        let response = TestHandler::handle_command(command);

        match response {
            Response::Test(s) => assert_eq!(s, input),
        }
    }
}

