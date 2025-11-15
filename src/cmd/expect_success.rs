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
    type Error = ExpectSuccessStatusError<C::Error>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        let status = self.inner.status().map_err(ExpectSuccessStatusError::Propagated)?;
        match status.success() {
            true => Ok(status),
            false => Err(ExpectSuccessStatusError::Unexpected(status)),
        }
    }
}

#[derive(Debug)]
pub enum ExpectSuccessStatusError<P> {
    Propagated(P),
    Unexpected(ExitStatus),
}

impl<C: OutputExt> OutputExt for ExpectSuccess<C> {
    type Error = ExpectSuccessOutputError<C::Error>;
    fn output(&mut self) -> Result<Output, Self::Error> {
        let output = self.inner.output().map_err(ExpectSuccessOutputError::Propagated)?;
        match output.status.success() {
            true => Ok(output),
            false => Err(ExpectSuccessOutputError::Unexpected(output)),
        }
    }
}

#[derive(Debug)]
pub enum ExpectSuccessOutputError<P> {
    Propagated(P),
    Unexpected(Output),
}
