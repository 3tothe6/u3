use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::Stdio,
};

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

impl<C: SpawnExt> StatusExt for Pretty<C> {
    fn status(&mut self) -> ExitStatus {
        let current_dir = self.raw().get_current_dir();
        let program = self.raw().get_program();
        let args = self.raw().get_args().collect::<Vec<_>>();
        let envs = self.raw().get_envs().collect::<HashMap<_, _>>();

        let span = tracing::info_span!("cmd", current_dir = ?current_dir, program = ?program, args = ?args, envs = ?envs);

        let mut child = self.inner.spawn();

        std::thread::scope(|s| {
            s.spawn(|| {
                let _enter = span.enter();
                BufReader::new(child.stdout.as_mut().unwrap()).lines().for_each(|l| {
                    tracing::info!(stdio = "stdout", message = l.unwrap());
                });
            });
            s.spawn(|| {
                let _enter = span.enter();
                BufReader::new(child.stderr.as_mut().unwrap()).lines().for_each(|l| {
                    tracing::info!(stdio = "stderr", message = l.unwrap());
                });
            });
        });

        child.wait().unwrap()
    }
}

pub struct PrettyOptions;

impl PrettyOptions {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PrettyOptions {
    fn default() -> Self {
        Self::new()
    }
}
