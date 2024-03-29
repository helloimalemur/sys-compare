pub mod comparemode;
pub mod createmode;
mod options;


use std::env::args;
use std::process::exit;
use clap::{FromArgMatches, Parser};
use crate::comparemode::CompareMode;
use crate::createmode::CreateMode;
use crate::options::{Arguments, Commands};

fn main() {
    let options = Arguments::parse();
    let movable = options.clone();

    let _app = match options.command {
        None => {}
        Some(Commands::Create { root_dir, output_path }) => {
            let mut create =
                CreateMode::new(output_path, root_dir);
            println!("Creating snapshot..");
            create.run()
        },
        Some(Commands::Compare { left, right, selection }) => {
            println!("Running snapshot comparison..");
            let mut compare = CompareMode::new(movable.clone(), left, right, selection);
            compare.run()
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
