use Fasching::snapshot::Snapshot;

pub enum CompareMode {
    Local2Local
}

pub struct SysCompareApp {
    mode: CompareMode,
    args: Vec<String>
}


impl SysCompareApp {
    pub fn new(mode: CompareMode, args: Vec<String>) -> SysCompareApp {
        SysCompareApp { mode, args }
    }
    pub fn run(&self) {
        println!("running");
    }
}

impl Default for SysCompareApp {
    fn default() -> Self {
        SysCompareApp { mode: CompareMode::Local2Local, args: vec![] }
    }
}

pub trait Comparer {
    fn run(&self);
}
