use std::collections::HashMap;
use std::env::args;
use std::sync::{Arc, Mutex};
use Fasching::snapshot::Snapshot;
use crate::comparemode::CompareMode;
use crate::createmode::CreateMode;

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
                let in_path = match self.args.get(2) {
                    None => {panic!("Missing hash dir path as second argument")}
                    Some(r) => {not_empty(r)}
                };
                let out_path = match self.args.get(3) {
                    None => {panic!("Missing output path as third argument")}
                    Some(r) => {not_empty(r)}
                };
                let create = CreateMode::new(self.args.clone(), in_path.clone(), out_path.clone());
                create.run()
            }
            SysCompareMode::Compare => {
                let left = match self.args.get(2) {
                    None => {panic!("Missing hash dir path as second argument")}
                    Some(r) => {not_empty(r)}
                };
                let right = match self.args.get(3) {
                    None => {panic!("Missing output path as third argument")}
                    Some(r) => {not_empty(r)}
                };

                let compare = CompareMode::new(self.args.clone(), left, right);
            }
        }
    }
}

fn not_empty(r: &String) -> String {
    if r.replace("./", "").is_empty() {
        panic!("Specify input file name")
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
    fn run(&self);
}
