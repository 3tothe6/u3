use std::path::Path;

use url::Url;

pub mod color_ext;

pub fn see_path<P: AsRef<Path>>(path: P) {
    let path = dunce::canonicalize(path).unwrap();
    println!("See {} ({}).", Url::from_file_path(&path).unwrap(), path.display());
}

pub struct IsNotFirstItem {
    pub v: bool,
}

impl IsNotFirstItem {
    pub fn new() -> Self {
        Self { v: false }
    }
    pub fn exec<F: FnOnce() -> Result<(), E>, E>(&mut self, f: F) -> Result<(), E> {
        match self.v {
            false => {
                self.v = true;
                Ok(())
            }
            true => f(),
        }
    }
}

impl Default for IsNotFirstItem {
    fn default() -> Self {
        Self::new()
    }
}
