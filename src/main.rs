pub mod syscompare;

use std::env::args;
use crate::syscompare::{SysCompare};
use crate::syscompare::CompareMode::Local2Local;

fn main() {
    let args: Vec<String> = args().collect();
    println!("{:#?}", args);

    let app = match args.get(1) {
        None => {
            panic!("Missing Mode Argument");
            SysCompare::default()
        }
        Some(mode) => {
            let m = mode.as_str();

            let app_mode = match m {
                "local" => {Local2Local}
                _ => {Local2Local}
            };

            SysCompare::new(app_mode)
        }
    };

}
