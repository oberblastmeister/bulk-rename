use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;
use tempfile::NamedTempFile;

use crate::filesystem::{convert_paths_to_string, get_sorted_paths};
use crate::regex::{create_regex, find_match};

pub struct EditorRename {
    editor: String,
    file: NamedTempFile,
    path_strs: Vec<String>,
}

impl EditorRename {
    pub fn new(pattern: Option<&String>) -> Result<EditorRename> {
        let editor = match env::var("EDITOR") {
            Ok(editor) => editor,
            Err(_) => String::from("vim"),
        };
        let mut file = NamedTempFile::new().context("Failed to create named tempfile")?;
        let paths = get_sorted_paths("./")?;
        let path_str_iter = convert_paths_to_string(paths).filter_map(|r| r.ok());
        let matches = if let Some(pattern) = pattern {
            let regex = create_regex(pattern)?;
            find_match(path_str_iter, regex)
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

    Bulkrename will default to vim if $EDITOR is not set.
    Please check if vim or $EDITOR exists on your system.",
                file_path.display(),
                &self.editor
            ))?;

        let ecode = child
            .wait()
            .context(format!("Failed to wait on child command {}", &self.editor))?;

        if !ecode.success() {
            return Err(anyhow!(
                "{} exited with a non-zero code. Not changing any files.",
                self.editor
            ));
        }

        Ok(())
    }

    /// rename all files using the tempfile that the user edited
    pub fn rename_using_file(&self) -> Result<()> {
        let contents = fs::read_to_string(self.file.path())?;
        let vec_contents = contents.lines().collect::<Vec<_>>();

        if self.path_strs.len() != vec_contents.len() {
            return Err(anyhow!(
                "Do not delete or add lines from the file, only change them."
            ));
        }

        self.path_strs
            .par_iter()
            .zip(vec_contents.par_iter())
            .for_each(|(f, t)| {
                if f != t {
                    fs::rename(f, t).unwrap();
                }
            });

        Ok(())
    }
}
