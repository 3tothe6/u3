use std::path::Path;

use url::Url;

pub mod color_ext;

pub fn see_path<P: AsRef<Path>>(path: P) {
    let path = path.as_ref().canonicalize().unwrap();
    println!("See {} ({}).", Url::from_file_path(&path).unwrap(), path.display());
}
