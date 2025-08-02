use std::fmt;

pub struct TestCommand(pub String);

pub enum Command {
    Test(TestCommand),
}

pub enum Response {
    Test(String),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Test(s) => write!(f, "{}", s)
        }
    }
}

