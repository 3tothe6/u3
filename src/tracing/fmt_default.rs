use std::io::IsTerminal;

use chrono::prelude::*;
use tracing_subscriber::fmt::{
    FormatFields, SubscriberBuilder,
    format::{Format, Writer},
    time::FormatTime,
};

use crate::chrono::ext::Ext;

pub trait WithU3Default<N, L, F, W> {
    fn with_u3_default(self) -> SubscriberBuilder<N, Format<L, Timer>, F, W>;
}

impl<N, L, T, F, W> WithU3Default<N, L, F, W> for SubscriberBuilder<N, Format<L, T>, F, W>
where
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn with_u3_default(self) -> SubscriberBuilder<N, Format<L, Timer>, F, W> {
        self.with_timer(Timer).with_ansi(std::io::stdout().is_terminal())
    }
}

pub struct Timer;

impl FormatTime for Timer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format_u3())
    }
}
