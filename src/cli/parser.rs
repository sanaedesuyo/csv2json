use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub save: Option<PathBuf>,

    #[arg(index = 1, required = true)]
    pub filename: PathBuf,
}