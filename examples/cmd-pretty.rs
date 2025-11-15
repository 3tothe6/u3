use std::process::Command;

use u3::{
    cmd::{BaseExt, OutputExt, StatusExt, StdCmdExt},
    tracing::fmt_default::WithU3Default,
};

fn main() {
    tracing_subscriber::fmt().with_u3_default().init();
    Command::new("cargo").ext().pretty_term().pause_on_failure().status().unwrap();
    Command::new("cargo").arg("cleaned").ext().pretty_term().pause_on_failure().output().unwrap();
    Command::new("cargo").ext().pretty_tracing().pause_on_failure().status().unwrap();
}
