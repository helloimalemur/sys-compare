use Fasching::{compare_snapshots, create_snapshot, import_snapshot};
use Fasching::hasher::HashType::BLAKE3;
use Fasching::snapshot::{Snapshot, SnapshotCompareResult};
use crate::syscompare::Comparer;

pub struct CompareMode {
    left: Snapshot,
    right: Snapshot,
    args: Vec<String>,
    results: SnapshotCompareResult
}

impl CompareMode {
    pub fn new(args: Vec<String>, left: String, right: String) -> CompareMode {

        let left = import_snapshot(left);
        let right = import_snapshot(right);


        CompareMode {
            left,
            right,
            args,
            results: SnapshotCompareResult {
                created: vec![],
                deleted: vec![],
                changed: vec![],
            },
        }
    }
}

impl Comparer for CompareMode {
    fn run(&self) {
        let results = compare_snapshots(self.left.clone(), self.right.clone()).unwrap();

    }
}
