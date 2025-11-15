use std::process::{ExitStatus, Output};

pub trait ExitStatusOrOutput {
    fn is_output(&self) -> bool;
    fn status(&self) -> ExitStatus;
}

impl ExitStatusOrOutput for ExitStatus {
    fn is_output(&self) -> bool {
        false
    }
    fn status(&self) -> ExitStatus {
        *self
    }
}

impl ExitStatusOrOutput for Output {
    fn is_output(&self) -> bool {
        true
    }
    fn status(&self) -> ExitStatus {
        self.status
    }
}

#[derive(Debug)]
pub enum ExpectXxxStatusError<P> {
    Propagated(P),
    Unexpected(ExitStatus),
}

#[derive(Debug)]
pub enum ExpectXxxOutputError<P> {
    Propagated(P),
    Unexpected(Output),
}
