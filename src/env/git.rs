use crate::{cmd, cmd_o};

pub fn working_tree_clean() -> bool {
    cmd_o!("git.exe", "status", "--porcelain").unwrap().stdout.is_empty()
}

pub fn with_working_tree_clean() -> WithWorkingTreeCleanGuard {
    let clean = working_tree_clean();
    if !clean {
        cmd!("git.exe", "stash", "push", "--include-untracked").unwrap();
    }
    WithWorkingTreeCleanGuard { clean }
}

pub struct WithWorkingTreeCleanGuard {
    clean: bool,
}

impl Drop for WithWorkingTreeCleanGuard {
    fn drop(&mut self) {
        if !self.clean {
            cmd!("git.exe", "stash", "pop", "--index").unwrap();
        }
    }
}
