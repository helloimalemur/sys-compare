use Fasching::snapshot::Snapshot;
use crate::syscompare::Comparer;

pub struct CreateMode {
    path: String,
    snapshot: Snapshot
}

impl CreateMode {
    pub fn new(path: &String) -> CreateMode {
        CreateMode { path: "".to_string(), snapshot: Default::default() }
    }
}

impl Comparer for CreateMode {
    fn run(&self) {

    }
}
