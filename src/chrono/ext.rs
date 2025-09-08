use chrono::prelude::*;

pub trait Ext {
    fn format_u3(&self) -> String;
}

impl<Tz> Ext for DateTime<Tz>
where
    Tz: TimeZone,
{
    fn format_u3(&self) -> String {
        self.to_rfc3339_opts(SecondsFormat::Micros, false)
    }
}
