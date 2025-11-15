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
    type Error = ExpectNoExitError<C::Error, ExitStatus>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        let status = self.inner.status().map_err(ExpectNoExitError::Propagated)?;
        Err(ExpectNoExitError::Unexpected(status))
    }
}

impl<C: OutputExt> OutputExt for ExpectNoExit<C> {
    type Error = ExpectNoExitError<C::Error, Output>;
    fn output(&mut self) -> Result<Output, Self::Error> {
        let output = self.inner.output().map_err(ExpectNoExitError::Propagated)?;
        Err(ExpectNoExitError::Unexpected(output))
    }
}

#[derive(Debug)]
pub enum ExpectNoExitError<P, V> {
    Propagated(P),
    Unexpected(V),
}
