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
        let selector = match self.args.get(4) {
            None => {"none"}
            Some(r) => { r }
        };

        let results = compare_snapshots(self.left.clone(), self.right.clone()).unwrap();
        self.results = results.1;
        self.result_type = results.0;


        match selector {
            "created" => {
                for file in self.results.created.iter() {
                    println!("{}", file);
                }
                println!("Created: {:?}", self.results.created.len());
            },
            "deleted" => {
                for file in self.results.deleted.iter() {
                    println!("{}", file);
                }
                println!("Deleted: {:?}", self.results.deleted.len());
            },
            "changed" => {
                for file in self.results.changed.iter() {
                    println!("{}", file);
                }
                println!("Changed: {:?}", self.results.changed.len());
            }
            "none" => {
                println!("Created: {:?}", self.results.created.len());
                println!("Deleted: {:?}", self.results.deleted.len());
                println!("Changed: {:?}", self.results.changed.len());
            },
            _ => {
                // println!("Created: {:?}", self.results.created.len());
                // println!("Deleted: {:?}", self.results.deleted.len());
                // println!("Changed: {:?}", self.results.changed.len());
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use std::fmt::format;
    use crate::comparemode::CompareMode;
    use crate::createmode::CreateMode;

    #[test]
    fn compare_mode() {
        let user = whoami::username();
        let left = format!("/home/{}/test1", user);
        let right = format!("/home/{}/test2", user);
        let n1 = CreateMode::new(vec![], left.clone(), right.clone());

        let cm = CompareMode::new(vec![], left.clone(), right.clone());
    }
}
