use std::process::{Command as StdCmd, ExitStatus};

use self::pretty::Pretty;

mod pretty;

pub trait StdCmdExt {
    fn ext(&mut self) -> Std<'_>;
}

pub struct Std<'a> {
    inner: &'a mut StdCmd,
}

pub trait StatusExt: Sized {
    fn status(&mut self) -> ExitStatus;

    fn pretty(self) -> Pretty<Self> {
        Pretty { inner: self }
    }
}

impl StatusExt for Std<'_> {
    fn status(&mut self) -> ExitStatus {
        self.inner.status().unwrap()
    }
}
