use std::process::Stdio;

use super::*;

pub struct Pretty<C> {
    inner: C,
    options: PrettyOptions,
}

impl<C: BaseExt> Pretty<C> {
    pub fn new(inner: C, options: PrettyOptions) -> Self {
        let mut inner = inner;
        inner.raw_mut().stdout(Stdio::piped()).stderr(Stdio::piped());
        Self { inner, options }
    }
}

impl<C: BaseExt> BaseExt for Pretty<C> {
    fn raw(&self) -> &StdCmd {
        self.inner.raw()
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner.raw_mut()
    }
}

impl<C: StatusExt> StatusExt for Pretty<C> {
    fn status(&mut self) -> ExitStatus {
        let current_dir = self.raw().get_current_dir();
        let program = self.raw().get_program();
        let args = self.raw().get_args().collect::<Vec<_>>();

        tracing::info!(current_dir = ?current_dir, program = ?program, args = ?args);

        let status = self.inner.status();

        status
    }
}

pub struct PrettyOptions {
    ansi: bool,
}

impl PrettyOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for PrettyOptions {
    fn default() -> Self {
        Self { ansi: true }
    }
}
