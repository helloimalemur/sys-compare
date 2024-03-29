use clap::{Command, Parser, Subcommand};


#[derive(Parser, Clone, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(short, long)]
    pub input_path: Option<String>,
    #[arg(short, long)]
    pub output_path: Option<String>,

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
    },
}
