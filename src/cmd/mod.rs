use std::process::{Command as StdCmd, ExitStatus};

use self::pretty::{Pretty, PrettyOptions};

mod pretty;

pub trait StdCmdExt {
    fn ext(&mut self) -> Std<'_>;
}

impl StdCmdExt for StdCmd {
    fn ext(&mut self) -> Std<'_> {
        Std { inner: self }
    }
}

pub struct Std<'a> {
    inner: &'a mut StdCmd,
}

pub trait BaseExt {
    fn raw(&self) -> &StdCmd;
    fn raw_mut(&mut self) -> &mut StdCmd;
}

pub trait StatusExt: BaseExt + Sized {
    fn status(&mut self) -> ExitStatus;

    fn pretty(self, options: PrettyOptions) -> Pretty<Self> {
        Pretty::new(self, options)
    }
}

impl BaseExt for Std<'_> {
    fn raw(&self) -> &StdCmd {
        self.inner
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner
    }
}

impl StatusExt for Std<'_> {
    fn status(&mut self) -> ExitStatus {
        self.inner.status().unwrap()
    }
}
