mod editor_rename;
mod errors;
mod exit_codes;
mod filesystem;
mod opt;
mod regex;
mod replace_rename;

use std::process;

use anyhow::{Context, Result};
use rayon::prelude::*;
use structopt::StructOpt;

use editor_rename::EditorRename;
use exit_codes::ExitCode;
use opt::Opt;
use replace_rename::ReplaceRename;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn try_main(opt: Opt) -> Result<()> {
    if let Some(replace) = opt.rename {
        let pattern = opt
            .pattern
            .context("the pattern must if supplied if you are using the rename option")?;
        let replace_rename = ReplaceRename::new(&pattern, &replace, opt.hidden)?;
        let replaced = replace_rename.replace();
        replace_rename
            .rename_using_replace(&replaced.par_iter().map(|s| &**s).collect::<Vec<_>>())?;
    } else {
        let editor_rename = EditorRename::new(opt.pattern.as_ref(), opt.hidden)?;
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
