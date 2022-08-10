pub use crate::command::Command;
pub use crate::types::{AppResult, Error, Message};

/// trait interface for a CTAPHID application.
/// The application chooses which commands to register to, and will be called upon
/// when the commands are received in the CTAPHID layer.  Only one application can be registered to a particular command.
pub trait App {
    /// Define which CTAPHID commands to register to.
    fn commands(&self) -> &'static [Command];

    fn peek(&self, request: &Message) -> bool {
        true
    }

    /// Application is called here when one of it's register commands occurs.
    /// Application must put response in @message, or decide to return an error.
    ///
    /// The response is pre-cleared.
    fn call(&mut self, command: Command, request: &Message, response: &mut Message) -> AppResult;
}
