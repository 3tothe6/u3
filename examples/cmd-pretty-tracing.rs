use std::process::Command;

use u3::{
    cmd::{SpawnExt, StatusExt, StdCmdExt},
    tracing::fmt_default::WithU3Default,
};

fn main() {
    tracing_subscriber::fmt().with_u3_default().init();
    Command::new("cargo").ext().pretty_tracing().status().unwrap();
}
