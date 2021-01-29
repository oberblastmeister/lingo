mod kinds_src;

use crate::utils;
use kinds_src::KindsSrc;

use std::path::Path;

use eyre::Result;
use xshell::{read_file, write_file};

pub fn run() -> Result<()> {
    let kinds_src = KindsSrc::get()?;

    let syntax_kinds_file =
        utils::project_root().join("crates/parser/src/syntax_kind/generated.rs");
    let syntax_kinds = kinds_src.gen_syntax_kinds()?;
    update(syntax_kinds_file.as_path(), &syntax_kinds)?;

    Ok(())
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
fn update(path: &Path, contents: &str) -> Result<()> {
    fn normalize(s: &str) -> String {
        s.replace("\r\n", "\n")
    }

    match read_file(path) {
        Ok(old_contents) if normalize(&old_contents) == normalize(contents) => {
            return Ok(());
        }
        _ => (),
    }

    eprintln!("updating {}", path.display());
    write_file(path, contents)?;
    Ok(())
}
