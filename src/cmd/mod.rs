use std::fmt::Debug;
use std::process::{Child, Command as StdCmd, ExitStatus, Output};
use std::string::FromUtf8Error;

mod commons;
mod expect_noexit;
mod expect_success;
mod pause_on_failure;
mod pretty_term;
mod pretty_tracing;

pub use self::expect_noexit::{ExpectNoExit, ExpectNoExitError};
pub use self::expect_success::{ExpectSuccess, ExpectSuccessError};
pub use self::pause_on_failure::PauseOnFailure;
pub use self::pretty_term::{PrettyTerm, PrettyTermError};
pub use self::pretty_tracing::{PrettyTracing, PrettyTracingStatusError};

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

pub trait SpawnExt: BaseExt {
    type Error: Debug;
    fn spawn(&mut self) -> Result<Child, Self::Error>;
}

pub trait StatusExt: BaseExt {
    type Error: Debug;
    fn status(&mut self) -> Result<ExitStatus, Self::Error>;
}

pub trait OutputExt: BaseExt {
    type Error: Debug;
    fn output(&mut self) -> Result<Output, Self::Error>;
    fn output_utf8(&mut self) -> Result<OutputUtf8, OutputUtf8Error<Self::Error>> {
        let Output { status, stdout, stderr } =
            self.output().map_err(OutputUtf8Error::Propagated)?;
        let stdout = match String::from_utf8(stdout) {
            Ok(stdout) => stdout,
            Err(e) => {
                let stdout = e.as_bytes().to_vec();
                return Err(OutputUtf8Error::FromUtf8(e, Output { status, stdout, stderr }));
            }
        };
        let stderr = match String::from_utf8(stderr) {
            Ok(stderr) => stderr,
            Err(e) => {
                let stdout = stdout.into_bytes();
                let stderr = e.as_bytes().to_vec();
                return Err(OutputUtf8Error::FromUtf8(e, Output { status, stdout, stderr }));
            }
        };
        Ok(OutputUtf8 { status, stdout, stderr })
    }
}

#[derive(Debug)]
pub struct OutputUtf8 {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug)]
pub enum OutputUtf8Error<P> {
    Propagated(P),
    FromUtf8(FromUtf8Error, Output),
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

#[macro_export]
macro_rules! cmd_o {
    ($program:expr $(, $arg:expr)* $(,)?) => {
        {
            use $crate::cmd::{BaseExt, OutputExt, StdCmdExt};
            ::std::process::Command::new($program)
                $(.arg($arg))*
                .ext()
                .expect_success()
                .output_utf8()
        }
    };
}
