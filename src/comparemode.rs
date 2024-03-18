use Fasching::{compare_snapshots, create_snapshot, import_snapshot};
use Fasching::hasher::HashType::BLAKE3;
use Fasching::snapshot::{Snapshot, SnapshotChangeType, SnapshotCompareResult};
use crate::syscompare::Comparer;

pub struct CompareMode {
    left: Snapshot,
    right: Snapshot,
    args: Vec<String>,
    result_type: SnapshotChangeType,
    results: SnapshotCompareResult,
}

impl CompareMode {
    pub fn new(args: Vec<String>, left: String, right: String) -> CompareMode {

        let left = import_snapshot(left);
        let right = import_snapshot(right);


        CompareMode {
            left,
            right,
            args,
            result_type: SnapshotChangeType::None,
            results: SnapshotCompareResult {
                created: vec![],
                deleted: vec![],
                changed: vec![],
            },
        }
    }
}

impl Comparer for CompareMode {
    fn run(&mut self) {
        let selector = match self.args.get(3) {
            None => {"none"}
            Some(r) => { r }
        };

        let results = compare_snapshots(self.left.clone(), self.right.clone()).unwrap();
        self.results = results.1;
        self.result_type = results.0;


        match selector {
            "created" => {
                println!("Created: {:?}", self.results.created.len());
                for file in self.results.created {
                    println!("{}", file);
                }
            },
            "deleted" => {
                println!("Deleted: {:?}", self.results.deleted.len());
                for file in self.results.deleted {
                    println!("{}", file);
                }
            },
            "changed" => {
                println!("Changed: {:?}", self.results.changed.len());
                for file in self.results.changed {
                    println!("{}", file);
                }
            }
            _ => {
                println!("Created: {:?}", self.results.created.len());
                println!("Deleted: {:?}", self.results.deleted.len());
                println!("Changed: {:?}", self.results.changed.len());
            }
        }
    }
}
