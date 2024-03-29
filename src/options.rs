use clap::{Parser, Subcommand};


#[derive(Parser, Clone, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Create a snapshot
    Create {
        /// Directory to create snapshot from
        #[arg(short, long)]
        root_dir: String,
        /// Snapshot output/save location
        #[arg(short, long)]
        output_path: String,
    },
    /// Compare two snapshots
    Compare {
        /// left side of diff
        #[arg(short, long)]
        left: String,
        /// right side of diff
        #[arg(short, long)]
        right: String,
        /// OPTIONAL: specify which change type specifically to return
        #[arg(short, long)]
        selection: Option<String>,
        /// OPTIONAL: when using selection specify to return count only or not
        #[arg(short)]
        count_only: Option<bool>,
    },
}
