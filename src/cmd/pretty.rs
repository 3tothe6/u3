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
        match self.options.mode {
            PrettyOptionsMode::Terminal => self.status_terminal(),
            PrettyOptionsMode::Tracing => self.status_tracing(),
        }
    }
}

impl<C: SpawnExt> Pretty<C> {
    fn status_terminal(&mut self) -> ExitStatus {
        todo!()
    }

    fn status_tracing(&mut self) -> ExitStatus {
        let current_dir = self.raw().get_current_dir();
        let program = self.raw().get_program();
        let args = self.raw().get_args().collect::<Vec<_>>();
        let envs = self.raw().get_envs().collect::<HashMap<_, _>>();
        let span = tracing::info_span!("cmd", current_dir = ?current_dir, program = ?program, args = ?args, envs = ?envs);
        let _entered = span.enter();

        let mut child = self.inner.spawn();
        tracing::info!(event = "spawn");

        std::thread::scope(|s| {
            s.spawn(|| {
                let _entered = span.enter();
                BufReader::new(child.stdout.as_mut().unwrap()).lines().for_each(|l| {
                    tracing::info!(event = "stdout", message = l.unwrap());
                });
            });
            s.spawn(|| {
                let _entered = span.enter();
                BufReader::new(child.stderr.as_mut().unwrap()).lines().for_each(|l| {
                    tracing::info!(event = "stderr", message = l.unwrap());
                });
            });
        });

        let status = child.wait().unwrap();
        tracing::info!(event = "exit", status = ?status);
        status
    }
}

pub struct PrettyOptions {
    mode: PrettyOptionsMode,
}

pub enum PrettyOptionsMode {
    Terminal,
    Tracing,
}

impl PrettyOptions {
    pub fn new() -> Self {
        Self { mode: PrettyOptionsMode::Terminal }
    }
    pub fn mode(self, mode: PrettyOptionsMode) -> Self {
        Self { mode }
    }
}

impl Default for PrettyOptions {
    fn default() -> Self {
        Self::new()
    }
}
