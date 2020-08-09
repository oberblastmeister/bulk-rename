use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// allow hidden directories to be shown
    #[structopt(short = "H", long)]
    pub hidden: bool,

    /// search through directories recursively
    #[structopt(short = "R", long)]
    pub recursive: bool,

    /// show error messages with more information
    #[structopt(short, long)]
    pub debug: bool,

    /// the pattern to search for
    pub pattern: Option<String>,

    /// What to rename a pattern to.
    /// The pattern option must be used if using this option
    pub rename: Option<String>,
}
