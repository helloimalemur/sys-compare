use Fasching::snapshot::Snapshot;
use crate::syscompare::Comparer;

struct LocalMode {
    left: Snapshot,
    right: Snapshot
}

impl LocalMode {
    fn new() -> LocalMode {
        LocalMode { left: Default::default(), right: Default::default() }
    }
}

impl Comparer for LocalMode {
    fn run(&self) {

    }
}
