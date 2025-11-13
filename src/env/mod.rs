use std::path::{Path, PathBuf};

pub mod git;

pub fn with_current_dir<P: AsRef<Path>>(path: P) -> WithCurrentDirGuard {
    let initial_current_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(path).unwrap();
    WithCurrentDirGuard { initial_current_dir }
}

pub struct WithCurrentDirGuard {
    initial_current_dir: PathBuf,
}

impl Drop for WithCurrentDirGuard {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.initial_current_dir).unwrap();
    }
}
