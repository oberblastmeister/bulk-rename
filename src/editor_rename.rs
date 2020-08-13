use std::env::var;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, ensure, Context, Result};
use tempfile::NamedTempFile;

use crate::filesystem::{bulk_rename, get_string_paths};
use crate::regex_helper::filter_matches;

/// Run editor rename.
pub fn run_editor_rename(pattern: Option<&str>, allow_hidden: bool) -> Result<()> {
    let editor = get_available_editors();
    let mut file = NamedTempFile::new().context("Failed to create named tempfile")?;
    let path_strs = filter_matches(get_string_paths("./", allow_hidden)?, pattern)?;

    write!(file, "{}", path_strs.join("\n")).context(format!(
        "Failed to write to temp file {}",
        file.path().display()
    ))?;

    open_editor(&editor, file.path())?;

    let contents = fs::read_to_string(file.path())?;
    let vec_contents = contents.lines().collect::<Vec<_>>();

    ensure!(
        path_strs.len() == vec_contents.len(),
        "Do not delete or add lines from the file, only change them."
    );

    if path_strs == vec_contents {
        return Ok(());
    }

    bulk_rename(&path_strs, &vec_contents)?;

    Ok(())
}

/// Get available editors according to environment variables $EDITOR and then $VISUAL. Uses vi as
/// default is those environment variables are not set.
fn get_available_editors() -> String {
    var("EDITOR").unwrap_or(var("VISUAL").unwrap_or(String::from("vi")))
}

/// Open the tempfile with $EDITOR, $VISUAL, or vi. This function returns weather the text-editor
/// exited successfuly.
pub fn open_editor(editor: &str, file_path: impl AsRef<Path>) -> Result<()> {
    let file_path = file_path.as_ref();
    let mut child = Command::new(editor)
        .arg(file_path)
        .spawn()
        .context(format!(
            "Failed to open {} with {}.

    Bulkrename will default to vi if $EDITOR or $VISUAL is not set.",
            file_path.display(),
            editor
        ))?;

    let ecode = child
        .wait()
        .context(format!("Failed to wait on child command {}", editor))?;

    if !ecode.success() {
        bail!(
            "{} exited with a non-zero code. Not changing any files.",
            editor
        );
    }

    Ok(())
}
