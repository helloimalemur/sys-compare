pub mod comparemode;
pub mod createmode;
mod options;

use crate::comparemode::CompareMode;
use crate::createmode::CreateMode;
use crate::options::{Arguments, Commands};
use clap::Parser;

fn main() {
    let options = Arguments::parse();

    let _app = match options.command {
        Commands::Create {
            root_dir,
            output_path,
            verbose,
        } => {
            let mut create = CreateMode::new(output_path, root_dir, verbose);
            println!("Creating snapshot..");
            create.run(verbose)
        }
        Commands::Compare {
            left,
            right,
            selection,
            count_only,
            verbose,
        } => {
            if let Some(count_only) = count_only {
                if !count_only {
                    println!("Running snapshot comparison..");
                }
            }
            let mut compare = CompareMode::new(left, right, selection, count_only, verbose);
            compare.run(verbose)
        }
    };
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test() {
//
//
//     }
// }
