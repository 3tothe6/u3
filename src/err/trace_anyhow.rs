use std::fmt::{Debug, Formatter};

pub trait TraceAnyhow {
    fn trace_anyhow<F: FnOnce(BetterDebug)>(self, f: F) -> Self;
}

impl<T> TraceAnyhow for anyhow::Result<T> {
    fn trace_anyhow<F: FnOnce(BetterDebug)>(self, f: F) -> Self {
        self.inspect_err(|e| f(BetterDebug { inner: e }))
    }
}

pub struct BetterDebug<'a> {
    inner: &'a anyhow::Error,
}

impl<'a> From<&'a anyhow::Error> for BetterDebug<'a> {
    fn from(inner: &'a anyhow::Error) -> Self {
        Self { inner }
    }
}

impl Debug for BetterDebug<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.inner).replace("\n\nCaused by:", "\nCaused by:"))
    }
}
