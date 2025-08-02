use mustash_core;
use mustash_core::commands::{Command, Response, TestCommand};
use mustash_core::dispatcher::Dispatcher;


#[test]
fn test_dispatch() {
    let mut dispatcher = Dispatcher::new();

    let cmd = Command::Test(TestCommand("hello".into()));

    let resp = dispatcher.dispatch(cmd);

    match resp {
        Response::Test(s) => assert_eq!(s, "hello")
    }
}

