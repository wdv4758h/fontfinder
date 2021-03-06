use std::env;
use std::fs::DirBuilder;
use std::io;
use std::path::{Path, PathBuf};

pub fn make_rec_dir(dir: &Path) -> io::Result<()> { DirBuilder::new().recursive(true).create(dir) }

pub fn font_cache() -> Option<PathBuf> {
    env::home_dir().map(|path| path.join(".local/share/fonts/"))
}

pub fn font_exists(base: &Path, family: &str, variant: &str, uri: &str) -> bool {
    get_font_path(base, family, variant, uri).exists()
}

pub fn get_font_path(base: &Path, family: &str, variant: &str, uri: &str) -> PathBuf {
    let extension = uri.rfind('.').map_or("", |pos| &uri[pos..]);
    base.join(&[family, "_", variant, extension].concat())
}
