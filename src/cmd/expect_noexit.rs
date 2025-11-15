use super::*;

pub struct ExpectNoExit<C> {
    inner: C,
}

impl<C> ExpectNoExit<C> {
    pub(super) fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C: BaseExt> BaseExt for ExpectNoExit<C> {
    fn raw(&self) -> &StdCmd {
        self.inner.raw()
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner.raw_mut()
    }
}

impl<C: StatusExt> StatusExt for ExpectNoExit<C> {
    type Error = ExpectNoExitStatusError<C::Error>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        let status = self.inner.status().map_err(ExpectNoExitStatusError::Propagated)?;
        Err(ExpectNoExitStatusError::Unexpected(status))
    }
}

#[derive(Debug)]
pub enum ExpectNoExitStatusError<P> {
    Propagated(P),
    Unexpected(ExitStatus),
}

impl<C: OutputExt> OutputExt for ExpectNoExit<C> {
    type Error = ExpectNoExitOutputError<C::Error>;
    fn output(&mut self) -> Result<Output, Self::Error> {
        let output = self.inner.output().map_err(ExpectNoExitOutputError::Propagated)?;
        Err(ExpectNoExitOutputError::Unexpected(output))
    }
}

#[derive(Debug)]
pub enum ExpectNoExitOutputError<P> {
    Propagated(P),
    Unexpected(Output),
}
