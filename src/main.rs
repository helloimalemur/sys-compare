pub mod comparemode;
pub mod createmode;
pub mod syscompare;
mod options;


use std::env::args;
use std::process::exit;
use clap::{FromArgMatches, Parser};
use crate::comparemode::CompareMode;
use crate::createmode::CreateMode;
use crate::options::{Arguments, Commands};
use crate::syscompare::Comparer;

fn main() {
    let options = Arguments::parse();
    let movable = options.clone();
    let args: Vec<String> = args().collect();
    // println!("{:#?}", args); // testing

    let app = match options.command {
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

pub fn print_help() {
    println!("### Create Snapshot\n## ./sys-compare create [snapshot] [root_dir]");
    println!("### Compare Snapshots\n## ./sys-compare compare [.snap] [.snap] [created]|[deleted]|[changed]");
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test() {
//
//
//     }
// }
