use clap::{Command, Parser, Subcommand};
use crate::syscompare::SysCompareMode;

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub show: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        root_dir: String,
        #[arg(short, long)]
        output_path: String,
    },
    Compare {
        #[arg(short, long)]
        left: String,
        #[arg(short, long)]
        right: String,
    },
}
