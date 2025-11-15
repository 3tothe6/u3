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
    type Error = C::Error;
    fn status(&mut self) -> Result<ExitStatus, C::Error> {
        let status = self.inner.status()?;
        if !status.success() {
            println!("Press enter to continue...");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
        }
        Ok(status)
    }
}
