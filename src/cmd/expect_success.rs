use super::{
    commons::{OutputError, StatusError},
    *,
};

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
    type Error = StatusError<C::Error>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        let status = self.inner.status().map_err(StatusError::Propagated)?;
        if status.success() { Ok(status) } else { Err(StatusError::Unexpected(status)) }
    }
}

impl<C: OutputExt> OutputExt for ExpectSuccess<C> {
    type Error = OutputError<C::Error>;
    fn output(&mut self) -> Result<Output, Self::Error> {
        let output = self.inner.output().map_err(OutputError::Propagated)?;
        if output.status.success() { Ok(output) } else { Err(OutputError::Unexpected(output)) }
    }
}
