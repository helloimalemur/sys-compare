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
        /// OPTIONAL: Increase verbosity
        #[arg(short, default_value="false", num_args = 0..2)]
        verbose: Option<bool>,
    },
    /// Compare two snapshots
    Compare {
        /// Left side of diff
        #[arg(short, long)]
        left: String,
        /// Right side of diff
        #[arg(short, long)]
        right: String,
        /// OPTIONAL: Wpecify which change type specifically to return
        #[arg(short, long)]
        selection: Option<String>,
        /// OPTIONAL: When using selection specify to return count only or not
        #[arg(short, default_value="false", num_args = 0..2)]
        count_only: Option<bool>,
        /// OPTIONAL: Increase verbosity
        #[arg(short, default_value="false", num_args = 0..2)]
        verbose: Option<bool>,
    },
}
