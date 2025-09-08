use std::fmt::Display;

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
};

pub trait Ext {
    fn format_u3(&self) -> String;
    fn format_for_filename(&self) -> DelayedFormat<StrftimeItems<'static>>;
}

impl<Tz> Ext for DateTime<Tz>
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    fn format_u3(&self) -> String {
        self.to_rfc3339_opts(SecondsFormat::Micros, false)
    }
    fn format_for_filename(&self) -> DelayedFormat<StrftimeItems<'static>> {
        self.format("%Y%m%d-%H%M%S-%6f")
    }
}
