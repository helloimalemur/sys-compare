use std::collections::HashMap;
use std::env::args;
use std::sync::{Arc, Mutex};
use Fasching::snapshot::Snapshot;
use crate::createmode::CreateMode;

pub enum CompareMode {
    Create,
    Compare
}

pub struct SysCompareApp {
    mode: CompareMode,
    args: Vec<String>,
    comparatives: Arc<Mutex<HashMap<String, Snapshot>>>
}


impl SysCompareApp {
    pub fn new(mode: CompareMode, args: Vec<String>) -> SysCompareApp {
        SysCompareApp { mode, args, comparatives: Arc::new(Mutex::new(HashMap::new())) }
    }
    pub fn run(&self) {
        println!("running");
        match self.mode {
            CompareMode::Create => {
                let in_path = match self.args.get(2) {
                    None => {panic!("Missing hash dir path as second argument")}
                    Some(r) => {
                        if r.replace("./", "").is_empty() {
                            panic!("Specify input file name")
                        } else {
                            r
                        }
                    }
                };
                let out_path = match self.args.get(3) {
                    None => {panic!("Missing output path as third argument")}
                    Some(r) => {
                        if r.replace("./", "").is_empty() {
                            panic!("Specify out path file name")
                        } else {
                            r
                        }
                    }
                };
                let create = CreateMode::new(self.args.clone(), in_path.clone(), out_path.clone());
                create.run()
            }
            CompareMode::Compare => {

            }
        }
    }
}

impl Default for SysCompareApp {
    fn default() -> Self {
        SysCompareApp { mode: CompareMode::Create, args: vec![], comparatives: Arc::new(Mutex::new(HashMap::new())) }
    }
}

pub trait Comparer {
    fn run(&self);
}
