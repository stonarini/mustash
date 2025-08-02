#[cfg(feature = "dispatcher")]
use mustash_macros::dispatcher;
#[cfg(feature = "dispatcher")]
use crate::handlers::Handler;

use crate::commands::{Command, Response};
pub struct Dispatcher;

#[cfg(not(feature = "dispatcher"))]
impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher
    }

    pub fn dispatch(&mut self, _cmd: Command) -> Response {
        unimplemented!("Stub dispatcher — enable `dispatcher` to use generated one.")
    }
}

#[cfg(feature = "dispatcher")]
#[dispatcher]
impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher
    }
}


