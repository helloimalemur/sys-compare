use clap::{Parser, Subcommand};


#[derive(Parser, Clone, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
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
        #[arg(short, long)]
        selection: Option<String>,
        #[arg(short, long)]
        count_only: Option<bool>,
    },
}
