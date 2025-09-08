use std::convert::Infallible;
use std::fmt::Debug;
use std::process::{Child, Command as StdCmd, ExitStatus, Output};

use self::expect_noexit::ExpectNoExit;
use self::expect_success::ExpectSuccess;
use self::pretty_term::PrettyTerm;
use self::pretty_tracing::PrettyTracing;

mod commons;
mod expect_noexit;
mod expect_success;
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

pub trait BaseExt: Sized {
    fn raw(&self) -> &StdCmd;
    fn raw_mut(&mut self) -> &mut StdCmd;

    fn pretty_tracing(self) -> PrettyTracing<Self> {
        PrettyTracing::new(self)
    }
    fn pretty_term(self) -> PrettyTerm<Self> {
        PrettyTerm::new(self)
    }
    fn expect_noexit(self) -> ExpectNoExit<Self> {
        ExpectNoExit::new(self)
    }
    fn expect_success(self) -> ExpectSuccess<Self> {
        ExpectSuccess::new(self)
    }
}

pub trait SpawnExt: BaseExt + Sized {
    type Error: Debug;
    fn spawn(&mut self) -> Result<Child, Self::Error>;
}

pub trait StatusExt: BaseExt + Sized {
    type Error: Debug;
    fn status(&mut self) -> Result<ExitStatus, Self::Error>;
}

pub trait OutputExt: BaseExt + Sized {
    type Error: Debug;
    fn output(&mut self) -> Result<Output, Self::Error>;
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
    type Error = std::io::Error;
    fn spawn(&mut self) -> Result<Child, Self::Error> {
        self.inner.spawn()
    }
}

impl StatusExt for StdCmdWrapper<'_> {
    type Error = std::io::Error;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        self.inner.status()
    }
}

impl OutputExt for StdCmdWrapper<'_> {
    type Error = std::io::Error;
    fn output(&mut self) -> Result<Output, Self::Error> {
        self.inner.output()
    }
}
