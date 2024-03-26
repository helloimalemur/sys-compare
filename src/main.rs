pub mod comparemode;
pub mod createmode;
pub mod syscompare;

use crate::syscompare::SysCompareApp;
use crate::syscompare::SysCompareMode::{Compare, Create};
use std::env::args;
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();
    // println!("{:#?}", args); // testing

    let app = match args.get(1) {
        None => {
            print_help();
            exit(0);
        }
        Some(mode) => {
            // app mode
            let m = mode.as_str();
            let app_mode = match m {
                "create" => Create,
                "compare" => Compare,
                _ => {
                    println!("Invalid MODE argument");
                    print_help();
                    exit(0);
                }
            };

            SysCompareApp::new(app_mode, args)
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
