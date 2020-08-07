mod exit_codes;
mod filesystem;
mod opt;

use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{self, Command};

use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;
use structopt::StructOpt;
use tempfile::NamedTempFile;

use exit_codes::ExitCode;
use filesystem::{convert_paths_to_string, get_sorted_paths};
use opt::Opt;

fn try_main(opt: Opt) -> Result<()> {
    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => String::from("vim"),
    };

    let mut file = NamedTempFile::new().context("Failed to create named tempfile")?;

    let paths = get_sorted_paths("./")?;
    write!(
        file,
        "{}",
        convert_paths_to_string(paths)
            .collect::<Result<Vec<String>>>()?
            .join("\n")
    )
    .context(format!(
        "Failed to write to temp file {}",
        file.path().display()
    ))?;

    let file_path = file.path();

    if let Some(pattern) = opt.pattern {
        let rename = opt.rename.context("--rename must be supplied if you are renaming with --pattern ")?;
    } else {
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
    }

    let contents = fs::read_to_string(file_path)?;
    println!("File content:\n{}", contents);

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let debug_mode = opt.debug;

    match try_main(opt) {
        Ok(()) => process::exit(ExitCode::Success.into()),
        Err(e) => {
            if debug_mode {
                eprintln!("[bulk-rename error]: {:?}", e);
            } else {
                eprintln!("[bulk-rename error]: {}", e);
            }
            process::exit(ExitCode::GeneralError.into())
        }
    }
}
