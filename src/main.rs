mod bulk_rename;
mod exit_codes;
mod filesystem;
mod opt;
mod regex;

use std::process;

use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;
use structopt::StructOpt;

use bulk_rename::EditorRename;
use exit_codes::ExitCode;
use opt::Opt;

fn try_main(opt: Opt) -> Result<()> {
    if let Some(rename) = opt.rename {
        let pattern = opt
            .pattern
            .context("--rename must be supplied if you are renaming with --pattern ")?;
    } else {
        let editor_rename = EditorRename::new(opt.pattern.as_ref())?;
        editor_rename.open_editor()?;
        editor_rename.rename_using_file()?;
    }

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
