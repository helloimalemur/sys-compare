pub mod syscompare;
mod localmode;

use std::env::args;
use crate::syscompare::{SysCompareApp};
use crate::syscompare::CompareMode::Local2Local;

fn main() {
    let args: Vec<String> = args().collect();
    println!("{:#?}", args);

    let app = match args.get(1) {
        None => {
            panic!("Missing Mode Argument");
            SysCompareApp::default()
        }
        Some(mode) => {
            // app mode
            let m = mode.as_str();
            let app_mode = match m {
                "local" => {Local2Local}
                _ => {panic!("Invalid MODE argument")}
            };

            SysCompareApp::new(app_mode, args)
        }
    };

    app.run()
}
