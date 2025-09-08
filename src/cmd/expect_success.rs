use super::*;

pub struct ExpectSuccess<C> {
    inner: C,
}

impl<C> ExpectSuccess<C> {
    pub(super) fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C: BaseExt> BaseExt for ExpectSuccess<C> {
    fn raw(&self) -> &StdCmd {
        self.inner.raw()
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner.raw_mut()
    }
}

impl<C: StatusExt> StatusExt for ExpectSuccess<C> {
    type Error = ErrorWithStatus;
    fn status(&mut self) -> Result<ExitStatus, ErrorWithStatus> {
        let status = self.inner.status().unwrap();
        if status.success() { Ok(status) } else { Err(ErrorWithStatus(status)) }
    }
}

impl<C: OutputExt> OutputExt for ExpectSuccess<C> {
    type Error = ErrorWithOutput;
    fn output(&mut self) -> Result<Output, ErrorWithOutput> {
        let output = self.inner.output().unwrap();
        if output.status.success() { Ok(output) } else { Err(ErrorWithOutput(output)) }
    }
}

#[derive(Debug)]
pub struct ErrorWithStatus(ExitStatus);

#[derive(Debug)]
pub struct ErrorWithOutput(Output);
