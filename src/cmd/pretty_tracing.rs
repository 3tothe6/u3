use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::Stdio,
    sync::Arc,
};

use chrono::prelude::*;

use crate::chrono::ext::Ext;

use super::*;

pub struct PrettyTracing<C> {
    inner: C,
}

impl<C: BaseExt> PrettyTracing<C> {
    pub(super) fn new(inner: C) -> Self {
        let mut inner = inner;
        inner.raw_mut().stdout(Stdio::piped()).stderr(Stdio::piped());
        Self { inner }
    }
}

impl<C: BaseExt> BaseExt for PrettyTracing<C> {
    fn raw(&self) -> &StdCmd {
        self.inner.raw()
    }
    fn raw_mut(&mut self) -> &mut StdCmd {
        self.inner.raw_mut()
    }
}

impl<C: SpawnExt> PrettyTracing<C> {
    pub fn spawn(&mut self) -> Result<Child, C::Error> {
        self.spawn_and_then(Ok, |e| e)
    }
}

impl<C: SpawnExt> StatusExt for PrettyTracing<C> {
    type Error = PrettyTracingStatusError<C::Error>;
    fn status(&mut self) -> Result<ExitStatus, Self::Error> {
        self.spawn_and_then(
            |mut child| {
                let status = child.wait().map_err(PrettyTracingStatusError::Wait)?;
                tracing::info!(event = "exit", status = ?status);
                Ok(status)
            },
            PrettyTracingStatusError::Spawn,
        )
    }
}

#[derive(Debug)]
pub enum PrettyTracingStatusError<S> {
    Spawn(S),
    Wait(std::io::Error),
}

impl<C: SpawnExt> PrettyTracing<C> {
    pub fn spawn_and_then<F, T, E, M>(&mut self, f: F, map_spawn_error: M) -> Result<T, E>
    where
        F: FnOnce(Child) -> Result<T, E>,
        M: FnOnce(C::Error) -> E,
    {
        let current_dir = self.raw().get_current_dir();
        let program = self.raw().get_program();
        let args = self.raw().get_args().collect::<Vec<_>>();
        let envs = self.raw().get_envs().collect::<HashMap<_, _>>();
        let span = Arc::new(tracing::info_span!(
            "cmd",
            current_dir = ?current_dir,
            program = ?program,
            args = ?args,
            envs = ?envs,
            date = ?Local::now().format_u3(),
        ));
        let _entered = span.enter();

        let mut child = self.inner.spawn().map_err(map_spawn_error)?;
        tracing::info!(event = "spawn");

        let span1 = span.clone();
        let span2 = span.clone();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        std::thread::spawn(move || {
            let _entered = span1.enter();
            BufReader::new(stdout).lines().for_each(|l| {
                tracing::info!(event = "stdout", message = l.unwrap());
            });
        });
        std::thread::spawn(move || {
            let _entered = span2.enter();
            BufReader::new(stderr).lines().for_each(|l| {
                tracing::info!(event = "stderr", message = l.unwrap());
            });
        });

        f(child)
    }
}
