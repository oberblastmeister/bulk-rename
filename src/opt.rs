use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long)]
    pub debug: bool,

    // #[structopt(short, long)]
    pub pattern: Option<String>,

    // #[structopt(short, long)]
    pub rename: Option<String>,

    #[structopt(short = "R", long)]
    pub recursive: bool,
}
