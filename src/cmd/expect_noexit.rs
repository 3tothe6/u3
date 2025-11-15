use super::{
    commons::{ExpectXxxOutputError, ExpectXxxStatusError},
    *,
};

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
    type Error = ExpectXxxStatusError<C::Error>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        let status = self.inner.status().map_err(ExpectXxxStatusError::Propagated)?;
        Err(ExpectXxxStatusError::Unexpected(status))
    }
}

impl<C: OutputExt> OutputExt for ExpectNoExit<C> {
    type Error = ExpectXxxOutputError<C::Error>;
    fn output(&mut self) -> Result<Output, Self::Error> {
        let output = self.inner.output().map_err(ExpectXxxOutputError::Propagated)?;
        Err(ExpectXxxOutputError::Unexpected(output))
    }
}
