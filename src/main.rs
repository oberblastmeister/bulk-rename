mod editor_rename;
mod errors;
mod exit_codes;
mod filesystem;
mod opt;
mod regex_helper;
mod replace_rename;

use std::env;
use std::process;

use anyhow::{Context, Result};
use rayon::prelude::*;
use structopt::StructOpt;

use errors::print_error;
use editor_rename::run_editor_rename;
use exit_codes::ExitCode;
use opt::Opt;
use replace_rename::ReplaceRename;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn try_main(opt: Opt) -> Result<()> {
    // set working directory
    if let Some(directory) = opt.directory {
        env::set_current_dir(&directory).context(format!(
            "Failed to change working directory to {}",
            &directory.display()
        ))?;
    }

    // decide if using replace rename or editor rename
    if let Some(replace) = opt.rename {
        let pattern = opt
            .pattern
            .context("the pattern must if supplied if you are using the rename option")?;
        let replace_rename = ReplaceRename::new(&pattern, replace, opt.hidden)?;
        let replaced = replace_rename.replace();
        replace_rename
            .rename_using_replace(&replaced.par_iter().map(|s| &**s).collect::<Vec<_>>())?;
    } else {
        run_editor_rename(opt.pattern.as_deref(), opt.hidden)?;
    }

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    match try_main(opt) {
        Ok(()) => process::exit(ExitCode::Success.into()),
        Err(e) => {
            print_error(e.to_string());
            process::exit(ExitCode::GeneralError.into())
        }
    }
}
