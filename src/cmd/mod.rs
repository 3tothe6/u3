use std::process::{Child, Command as StdCmd, ExitStatus, Output};

use self::pretty_term::PrettyTerm;
use self::pretty_tracing::PrettyTracing;

mod commons;
mod pretty_term;
mod pretty_tracing;

pub trait StdCmdExt {
    fn ext(&mut self) -> StdCmdWrapper<'_>;
}

impl StdCmdExt for StdCmd {
    fn ext(&mut self) -> StdCmdWrapper<'_> {
        StdCmdWrapper { inner: self }
    }
}

pub struct StdCmdWrapper<'a> {
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

    fn pretty_term(self) -> PrettyTerm<Self> {
        PrettyTerm::new(self)
    }
}

pub trait OutputExt: BaseExt + Sized {
    fn output(&mut self) -> anyhow::Result<Output>;

    fn pretty_term(self) -> PrettyTerm<Self> {
        PrettyTerm::new(self)
    }
}

impl BaseExt for StdCmdWrapper<'_> {
    fn raw(&self) -> &StdCmd {
        self.inner
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner
    }
}

impl SpawnExt for StdCmdWrapper<'_> {
    fn spawn(&mut self) -> Child {
        self.inner.spawn().unwrap()
    }
}

impl StatusExt for StdCmdWrapper<'_> {
    fn status(&mut self) -> anyhow::Result<ExitStatus> {
        Ok(self.inner.status().unwrap())
    }
}

impl OutputExt for StdCmdWrapper<'_> {
    fn output(&mut self) -> anyhow::Result<Output> {
        Ok(self.inner.output().unwrap())
    }
}
