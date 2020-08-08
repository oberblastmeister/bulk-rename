use std::env::var;
use std::fs;
use std::io::Write;
use std::process::Command;

use anyhow::{bail, ensure, Context, Result};
use rayon::prelude::*;
use tempfile::NamedTempFile;

use crate::filesystem::{bulk_rename, get_string_paths_iter};
use crate::regex::{create_lazy_regex, filter_matches};

pub struct EditorRename {
    editor: String,
    file: NamedTempFile,
    path_strs: Vec<String>,
}

impl EditorRename {
    pub fn new(pattern: Option<&String>) -> Result<EditorRename> {
        let editor = var("EDITOR").unwrap_or(var("VISUAL").unwrap_or(String::from("vi")));

        let mut file = NamedTempFile::new().context("Failed to create named tempfile")?;
        // get paths in dir and only get the ones that are utf-8 strings
        let path_str_iter = get_string_paths_iter("./")?;

        let matches = if let Some(pattern) = pattern {
            let regex = create_lazy_regex(pattern)?;
            filter_matches(path_str_iter, regex)
        } else {
            path_str_iter.collect()
        };

        write!(file, "{}", matches.join("\n")).context(format!(
            "Failed to write to temp file {}",
            file.path().display()
        ))?;

        Ok(EditorRename {
            editor,
            file,
            path_strs: matches,
        })
    }

    /// open file with $EDITOR or vim.
    /// Return weather it exited successfully
    pub fn open_editor(&self) -> Result<()> {
        let file_path = self.file.path();
        let mut child = Command::new(&self.editor)
            .arg(file_path)
            .spawn()
            .context(format!(
                "Failed to open {} with {}.

    Bulkrename will default to vi if $EDITOR or $VISUAL is not set.",
                file_path.display(),
                &self.editor
            ))?;

        let ecode = child
            .wait()
            .context(format!("Failed to wait on child command {}", &self.editor))?;

        if !ecode.success() {
            bail!(
                "{} exited with a non-zero code. Not changing any files.",
                self.editor
            );
        }

        Ok(())
    }

    /// rename all files using the tempfile that the user edited
    pub fn rename_using_file(&self) -> Result<()> {
        let contents = fs::read_to_string(self.file.path())?;
        let vec_contents = contents.lines().collect::<Vec<_>>();

        ensure!(
            self.path_strs.len() == vec_contents.len(),
            "Do not delete or add lines from the file, only change them."
        );

        let errors = bulk_rename(&self.path_strs, &vec_contents);
        if !errors.is_empty() {
            // join errors into one big error
            bail!(
                "\n{}",
                errors
                    .into_par_iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<_>>()
                    .join("\n\n[bulk-rename error]:\n")
            );
        }

        Ok(())
    }
}
