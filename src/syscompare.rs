use Fasching::snapshot::Snapshot;

pub enum CompareMode {
    Local2Local
}

pub struct SysCompare {
    mode: CompareMode,
    left: Snapshot,
    right: Snapshot,
}


impl SysCompare {
    pub fn new(mode: CompareMode) -> SysCompare {
        SysCompare { mode: CompareMode::Local2Local, left: Default::default(), right: Default::default() }
    }
    pub fn compare() {}
}

impl Default for SysCompare {
    fn default() -> Self {
        SysCompare { mode: CompareMode::Local2Local, left: Default::default(), right: Default::default() }
    }
}
