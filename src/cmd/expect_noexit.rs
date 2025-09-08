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
    type Error = ErrorWithStatus;
    fn status(&mut self) -> Result<ExitStatus, ErrorWithStatus> {
        let status = self.inner.status().unwrap();
        Err(ErrorWithStatus(status))
    }
}

impl<C: OutputExt> OutputExt for ExpectNoExit<C> {
    type Error = ErrorWithOutput;
    fn output(&mut self) -> Result<Output, ErrorWithOutput> {
        let output = self.inner.output().unwrap();
        Err(ErrorWithOutput(output))
    }
}

#[derive(Debug)]
pub struct ErrorWithStatus(ExitStatus);

#[derive(Debug)]
pub struct ErrorWithOutput(Output);
