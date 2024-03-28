pub mod comparemode;
pub mod createmode;
pub mod syscompare;
mod options;

use crate::syscompare::SysCompareApp;
use crate::syscompare::SysCompareMode::{Compare, Create};
use std::env::args;
use std::process::exit;
use clap::{FromArgMatches, Parser};
use crate::options::{Arguments, Commands};

fn main() {
    let options = Arguments::parse();

    let args: Vec<String> = args().collect();
    // println!("{:#?}", args); // testing

    let app = match options.command {
        None => {
            print_help();
            exit(0);
        }
        Some(Commands::Create { root_dir, output_path }) => {
            // app mode

            // let app_mode = match m {
            //     "create" => Create,
            //     "compare" => Compare,
            //     _ => {
            //         println!("Invalid MODE argument");
            //         print_help();
            //         exit(0);
            //     }
            // };

            SysCompareApp::new(Create, args)
        },
        Some(Commands::Compare { left, right }) => {
            SysCompareApp::new(Compare, args)
        }
    };

    app.run()
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
