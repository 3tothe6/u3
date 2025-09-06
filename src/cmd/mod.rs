use std::process::{Child, Command as StdCmd, ExitStatus};

use self::pretty_tracing::PrettyTracing;

mod pretty_tracing;

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

    fn pretty_tracing(self) -> PrettyTracing<Self> {
        PrettyTracing::new(self)
    }
}

pub trait StatusExt: BaseExt + Sized {
    fn status(&mut self) -> anyhow::Result<ExitStatus>;
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
    fn status(&mut self) -> anyhow::Result<ExitStatus> {
        Ok(self.inner.status().unwrap())
    }
}
