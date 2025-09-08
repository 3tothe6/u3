use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::Stdio,
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

impl<C: SpawnExt> StatusExt for PrettyTracing<C> {
    fn status(&mut self) -> anyhow::Result<ExitStatus> {
        let current_dir = self.raw().get_current_dir();
        let program = self.raw().get_program();
        let args = self.raw().get_args().collect::<Vec<_>>();
        let envs = self.raw().get_envs().collect::<HashMap<_, _>>();
        let span = tracing::info_span!(
            "cmd",
            current_dir = ?current_dir,
            program = ?program,
            args = ?args,
            envs = ?envs,
            date = ?Local::now().format_u3(),
        );
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
        Ok(status)
    }
}
