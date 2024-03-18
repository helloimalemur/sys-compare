use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use Fasching::snapshot::Snapshot;

pub enum CompareMode {
    Local2Local
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
    }
}

impl Default for SysCompareApp {
    fn default() -> Self {
        SysCompareApp { mode: CompareMode::Local2Local, args: vec![], comparatives: Arc::new(Mutex::new(HashMap::new())) }
    }
}

pub trait Comparer {
    fn run(&self);
}
