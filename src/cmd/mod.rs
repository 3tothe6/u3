use std::process::{Child, Command as StdCmd, ExitStatus};

use self::pretty::Pretty;

mod pretty;

pub use pretty::{PrettyOptions, PrettyOptionsMode};

pub trait StdCmdExt {
    fn ext(&mut self) -> Std<'_>;
}

impl StdCmdExt for StdCmd {
    fn ext(&mut self) -> Std<'_> {
        Std { inner: self }
    }
}

pub struct Std<'a> {
    inner: &'a mut StdCmd,
}

pub trait BaseExt {
    fn raw(&self) -> &StdCmd;
    fn raw_mut(&mut self) -> &mut StdCmd;
}

pub trait SpawnExt: BaseExt + Sized {
    fn spawn(&mut self) -> Child;

    fn pretty(self, options: PrettyOptions) -> Pretty<Self> {
        Pretty::new(self, options)
    }
}

pub trait StatusExt: BaseExt + Sized {
    fn status(&mut self) -> ExitStatus;
}

impl BaseExt for Std<'_> {
    fn raw(&self) -> &StdCmd {
        self.inner
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner
    }
}

impl SpawnExt for Std<'_> {
    fn spawn(&mut self) -> Child {
        self.inner.spawn().unwrap()
    }
}

impl StatusExt for Std<'_> {
    fn status(&mut self) -> ExitStatus {
        self.inner.status().unwrap()
    }
}
