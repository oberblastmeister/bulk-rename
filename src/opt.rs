use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long)]
    pub pattern: String,

    #[structopt(short, long)]
    pub rename: String,

    #[structopt(short = "R", long)]
    pub recursive: bool,
}
