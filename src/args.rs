use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Parser)]
pub struct Args {
    pub input: String,
    #[arg(default_value = "output.pdf")]
    pub output_file: PathBuf,
}