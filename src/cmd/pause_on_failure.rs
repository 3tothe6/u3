use std::process::ExitStatus;

use super::StatusExt;

pub struct PauseOnFailure<C> {
    inner: C,
}

impl<C> PauseOnFailure<C> {
    pub(super) fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C: StatusExt> PauseOnFailure<C> {
    pub fn status(&mut self) -> Result<ExitStatus, C::Error> {
        let status = self.inner.status()?;
        if !status.success() {
            println!("Press enter to continue...");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
        }
        Ok(status)
    }
}
