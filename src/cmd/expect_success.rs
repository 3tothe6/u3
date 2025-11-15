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
    type Error = ExpectSuccessError<C::Error, ExitStatus>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        let status = self.inner.status().map_err(ExpectSuccessError::Propagated)?;
        match status.success() {
            true => Ok(status),
            false => Err(ExpectSuccessError::Unexpected(status)),
        }
    }
}

impl<C: OutputExt> OutputExt for ExpectSuccess<C> {
    type Error = ExpectSuccessError<C::Error, Output>;
    fn output(&mut self) -> Result<Output, Self::Error> {
        let output = self.inner.output().map_err(ExpectSuccessError::Propagated)?;
        match output.status.success() {
            true => Ok(output),
            false => Err(ExpectSuccessError::Unexpected(output)),
        }
    }
}

#[derive(Debug)]
pub enum ExpectSuccessError<P, V> {
    Propagated(P),
    Unexpected(V),
}
