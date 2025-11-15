use std::fmt::Debug;
use std::process::{Child, Command as StdCmd, ExitStatus, Output};

mod commons;
mod expect_noexit;
mod expect_success;
mod pause_on_failure;
mod pretty_term;
mod pretty_tracing;

pub use self::expect_noexit::ExpectNoExit;
pub use self::expect_success::ExpectSuccess;
pub use self::pause_on_failure::PauseOnFailure;
pub use self::pretty_term::PrettyTerm;
pub use self::pretty_tracing::PrettyTracing;

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
    fn pause_on_failure(self) -> PauseOnFailure<Self> {
        PauseOnFailure::new(self)
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

#[macro_export]
macro_rules! cmd {
    ($program:expr $(, $arg:expr)* $(,)?) => {
        {
            use $crate::cmd::{BaseExt, StatusExt, StdCmdExt};
            ::std::process::Command::new($program)
                $(.arg($arg))*
                .ext()
                .pretty_term()
                .pause_on_failure()
                .status()
        }
    };
}
