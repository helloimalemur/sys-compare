use std::collections::HashMap;
use std::env::args;
use std::process::exit;
use std::sync::{Arc, Mutex};
use Fasching::snapshot::Snapshot;
use crate::comparemode::CompareMode;
use crate::createmode::CreateMode;
use crate::print_help;

pub enum SysCompareMode {
    Create,
    Compare
}

pub struct SysCompareApp {
    mode: SysCompareMode,
    args: Vec<String>,
    comparatives: Arc<Mutex<HashMap<String, Snapshot>>>
}


impl SysCompareApp {
    pub fn new(mode: SysCompareMode, args: Vec<String>) -> SysCompareApp {
        SysCompareApp { mode, args, comparatives: Arc::new(Mutex::new(HashMap::new())) }
    }
    pub fn run(&self) {
        println!("running");
        match self.mode {
            SysCompareMode::Create => {
                let snapshot_path = match self.args.get(2) {
                    None => {
                        println!("Missing hash dir path as second argument");
                        print_help();
                        exit(0);
                    }
                    Some(r) => {not_empty(r)}
                };
                let root_dir = match self.args.get(3) {
                    None => {
                        println!("Missing hash dir path as second argument");
                        print_help();
                        exit(0);
                    }
                    Some(r) => {not_empty(r)}
                };
                let mut create = CreateMode::new(self.args.clone(), snapshot_path.clone(), root_dir.clone());
                create.run()
            }
            SysCompareMode::Compare => {
                let left = match self.args.get(2) {
                    None => {
                        println!("Missing hash dir path as second argument");
                        print_help();
                        exit(0);
                    }
                    Some(r) => {not_empty(r)}
                };
                let right = match self.args.get(3) {
                    None => {
                        println!("Missing output path as third argument");
                        print_help();
                        exit(0);
                    }
                    Some(r) => {not_empty(r)}
                };

                let mut compare = CompareMode::new(self.args.clone(), left, right);
                compare.run()
            }
        }
    }
}

fn not_empty(r: &String) -> String {
    if r.replace("./", "").is_empty() {
        println!("Specify input file name");
        print_help();
        exit(0);
    } else {
        r.to_string()
    }
}

impl Default for SysCompareApp {
    fn default() -> Self {
        SysCompareApp { mode: SysCompareMode::Create, args: vec![], comparatives: Arc::new(Mutex::new(HashMap::new())) }
    }
}

pub trait Comparer {
    fn run(&mut self);
}
