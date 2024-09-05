use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Encode {
    //     #[arg(short, long)]
    //     input: PathBuf,
    //     #[arg(short, long)]
    //     output: PathBuf,
    // },
    // Decode {
    //     #[arg(short, long)]
    //     input: PathBuf,
    //     #[arg(short, long)]
    //     output: PathBuf,
    // },
    #[command(about = "Counts characters from a specified file")]
    CharCount {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
