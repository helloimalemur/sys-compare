pub mod syscompare;
pub mod createmode;
pub mod comparemode;

use std::env::args;
use crate::syscompare::{SysCompareApp};
use crate::syscompare::SysCompareMode::{Compare, Create};

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
                "create" => { Create },
                "compare" => { Compare },
                _ => {panic!("Invalid MODE argument")}
            };

            SysCompareApp::new(app_mode, args)
        }
    };

    app.run()
}
