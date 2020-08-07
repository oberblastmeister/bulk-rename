use std::env;
use std::fs;
use std::io::{Write};
use std::process::Command;

use anyhow::{Context, Result, anyhow};
use rayon::prelude::*;
use tempfile::NamedTempFile;

use crate::filesystem::{convert_paths_to_string, get_sorted_paths};
use crate::regex::find_pattern;

// pub fn bulk_rename(from: &[String], to: &[&str]) -> Result<()> {
//     assert_eq!(from.len(), to.len());

//     from.into_par_iter()
//         .zip(to.into_par_iter())
//         .for_each(|(f, t)| fs::rename(f, t).unwrap());

//     Ok(())
// }

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
        let matches = find_pattern(path_str_iter, pattern);

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

    pub fn rename_using_file(&self) -> Result<()> {
        let contents = fs::read_to_string(self.file.path())?;
        let vec_contents = contents.lines().collect::<Vec<_>>();

        if self.path_strs.len() != vec_contents.len() {
            return Err(anyhow!(
                "Do not delete or add lines from the file, only change them."
            ));
        } else if self.path_strs == vec_contents {
            return Ok(())
        }

        self.path_strs.par_iter()
            .zip(vec_contents.par_iter())
            .for_each(|(f, t)| fs::rename(f, t).unwrap());

        Ok(())
    }
}
