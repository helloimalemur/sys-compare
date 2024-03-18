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
    pub fn new(args: Vec<String>, in_path: String, out_path: String) -> CompareMode {


        CompareMode {
            left: Default::default(),
            right: Default::default(),
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

    }
}
