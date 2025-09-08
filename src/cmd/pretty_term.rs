use std::io::Write;

use termcolor::{Color::*, ColorSpec, StandardStream};

use crate::term::color_ext::ColorExt;

use super::{commons::ExitStatusOrOutput, *};

pub struct PrettyTerm<C> {
    inner: C,
}

impl<C> PrettyTerm<C> {
    pub(super) fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C: BaseExt> BaseExt for PrettyTerm<C> {
    fn raw(&self) -> &StdCmd {
        self.inner.raw()
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner.raw_mut()
    }
}

impl<C: StatusExt> StatusExt for PrettyTerm<C> {
    type Error = Infallible;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        Ok(self.exec(|s| s.inner.status().unwrap()))
    }
}

impl<C: OutputExt> OutputExt for PrettyTerm<C> {
    type Error = Infallible;
    fn output(&mut self) -> Result<Output, Self::Error> {
        Ok(self.exec(|s| s.inner.output().unwrap()))
    }
}

impl<C: BaseExt> PrettyTerm<C> {
    fn exec<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
        T: ExitStatusOrOutput,
    {
        let mut stderr = StandardStream::stderr(Default::default());

        stderr.with_color(ColorSpec::new().set_bg(Some(Cyan)).set_fg(Some(Black)), |s| {
            let current_dir = self
                .raw()
                .get_current_dir()
                .map(|p| p.canonicalize().unwrap())
                .unwrap_or_else(|| std::env::current_dir().unwrap());
            write!(s, "{}", current_dir.display()).unwrap();
        });
        write!(stderr, " ").unwrap();
        stderr.with_color(ColorSpec::new().set_fg(Some(Cyan)), |s| {
            write!(s, "{:?}", self.raw()).unwrap();
        });
        writeln!(stderr).unwrap();

        let v = f(self);

        stderr.with_color(
            ColorSpec::new()
                .set_bg(Some(match v.status().success() {
                    true => Green,
                    false => Red,
                }))
                .set_fg(Some(Black)),
            |s| {
                let eo = format!(
                    " END OUTPUT {}{}",
                    match v.is_output() {
                        true => "(CAPTURED) ",
                        _ => "",
                    },
                    match (v.status().success(), v.status().code()) {
                        (false, Some(c)) => &format!("({c}) "),
                        _ => "",
                    }
                );
                write!(s, "{eo}").unwrap();
            },
        );
        writeln!(stderr).unwrap();

        v
    }
}
