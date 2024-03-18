use Fasching::snapshot::Snapshot;
use crate::syscompare::Comparer;

struct CreateMode {
    path: String,
    snapshot: Snapshot
}

impl CreateMode {
    fn new() -> CreateMode {
        CreateMode { path: "".to_string(), snapshot: Default::default() }
    }
}

impl Comparer for CreateMode {
    fn run(&self) {

    }
}
