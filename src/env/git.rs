use std::process::Command;

use crate::cmd;
use crate::cmd::{BaseExt, OutputExt, StdCmdExt};

pub fn with_clean_working_tree() -> WithCleanWorkingTreeGuard {
    let clean = Command::new("git.exe")
        .args(["status", "--porcelain"])
        .ext()
        .expect_success()
        .output()
        .unwrap()
        .stdout
        .is_empty();
    if clean {
        cmd!("git.exe", "stash", "push", "--include-untracked").unwrap();
    }
    WithCleanWorkingTreeGuard { clean }
}

pub struct WithCleanWorkingTreeGuard {
    clean: bool,
}

impl Drop for WithCleanWorkingTreeGuard {
    fn drop(&mut self) {
        if self.clean {
            cmd!("git.exe", "stash", "pop", "--index").unwrap();
        }
    }
}
