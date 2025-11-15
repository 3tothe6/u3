use std::io::Write;

use super::*;

use super::StatusExt;

pub struct PauseOnFailure<C> {
    inner: C,
}

impl<C> PauseOnFailure<C> {
    pub(super) fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C: BaseExt> BaseExt for PauseOnFailure<C> {
    fn raw(&self) -> &StdCmd {
        self.inner.raw()
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner.raw_mut()
    }
}

impl<C: StatusExt> StatusExt for PauseOnFailure<C> {
    type Error = PauseOnFailureStatusError<C::Error>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        let status = self.inner.status().map_err(PauseOnFailureStatusError::Propagated)?;
        if !status.success() {
            println!("Press enter to continue...");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).map_err(PauseOnFailureStatusError::Stdio)?;
        }
        Ok(status)
    }
}

#[derive(Debug)]
pub enum PauseOnFailureStatusError<P> {
    Propagated(P),
    Stdio(std::io::Error),
}

impl<C: OutputExt> OutputExt for PauseOnFailure<C> {
    type Error = PauseOnFailureOutputError<C::Error>;
    fn output(&mut self) -> Result<Output, Self::Error> {
        let output = self.inner.output().map_err(PauseOnFailureOutputError::Propagated)?;
        if !output.status.success() {
            println!(
                "--------------------------------- BEGIN STDOUT ---------------------------------",
            );
            std::io::stdout()
                .write_all(&output.stdout)
                .map_err(PauseOnFailureOutputError::Stdio)?;
            println!(
                "---------------------------------- END STDOUT ----------------------------------",
            );
            eprintln!(
                "--------------------------------- BEGIN STDERR ---------------------------------",
            );
            std::io::stderr()
                .write_all(&output.stderr)
                .map_err(PauseOnFailureOutputError::Stdio)?;
            eprintln!(
                "---------------------------------- END STDERR ----------------------------------",
            );
            println!("Press enter to continue...");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).map_err(PauseOnFailureOutputError::Stdio)?;
        }
        Ok(output)
    }
}

#[derive(Debug)]
pub enum PauseOnFailureOutputError<P> {
    Propagated(P),
    Stdio(std::io::Error),
}
