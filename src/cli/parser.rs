use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    version = "v0.1.0 alpha",
    author = "sanaedesuyo",
    about = ".csv -> .json Parser",
    long_about = "A CLI used to parse .csv files into .json files.",
)]
pub struct Args {
    #[arg(short)]
    pub save: Option<PathBuf>,

    #[arg(index = 1, required = true)]
    pub filename: PathBuf,
}
