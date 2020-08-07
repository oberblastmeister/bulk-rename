mod exit_codes;
mod filesystem;
mod opt;

use std::env::{self, temp_dir};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{self, Command};

use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;
use tempfile::NamedTempFile;

use exit_codes::ExitCode;
use opt::Opt;

fn try_main() -> Result<()> {
    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => String::from("vim"),
    };

    let file = NamedTempFile::new().context("Failed to create named tempfile")?;
    let file_path = file.path();

    let mut child = Command::new(&editor)
        .arg(&file_path)
        .spawn()
        .context(format!(
            "Failed to open {} with {}.

Bulkrename will default to vim if $EDITOR is not set.
Please check if vim or $EDITOR exists on your system.",
            &file_path.display(),
            editor
        ))?;

    let ecode = child
        .wait()
        .context(format!("Failed to wait on child command {}", &editor))?;
    if !ecode.success() {
        return Err(anyhow!(
            "{} exited with a non-zero code. Not changing any files."
        ));
    }

    let contents = fs::read_to_string(file_path)?;
    println!("File content:\n{}", contents);

    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => process::exit(ExitCode::Success.into()),
        Err(e) => {
            eprintln!("[bulk-rename error]: {}", e);
            process::exit(ExitCode::GeneralError.into())
        }
    }
}
