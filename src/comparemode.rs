use crate::syscompare::Comparer;
use Fasching::snapshot::{Snapshot, SnapshotChangeType, SnapshotCompareResult};
use Fasching::{compare_snapshots, import_snapshot};

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
            None => "none",
            Some(r) => r,
        };

        let results = match compare_snapshots(self.left.clone(), self.right.clone()) {
            Some(x) => x,
            None => panic!("Error Comparing Snapshot"),
        };
        self.results = results.1;
        self.result_type = results.0;

        match selector {
            "created" => {
                self.results.created.iter().for_each(|e| println!("{e}"));
                println!("Created: {:?}", self.results.created.len());
            }
            "deleted" => {
                self.results.deleted.iter().for_each(|e| println!("{e}"));
                println!("Deleted: {:?}", self.results.deleted.len());
            }
            "changed" => {
                self.results.changed.iter().for_each(|e| println!("{e}"));
                println!("Changed: {:?}", self.results.changed.len());
            }
            "none" => {
                println!("Created: {:?}", self.results.created.len());
                println!("Deleted: {:?}", self.results.deleted.len());
                println!("Changed: {:?}", self.results.changed.len());
            }
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
    use crate::comparemode::CompareMode;
    use crate::createmode::CreateMode;
    use crate::syscompare::Comparer;
    use std::env;
    use std::fmt::format;

    #[test]
    fn compare_mode() {
        let user = whoami::username();
        println!("{user}");

        let left = format!("/home/{}/test1", user);
        let left_dir = format!("/home/{}/Documents/", user);
        println!("{left}");
        let right = format!("/home/{}/test2", user);
        let right_dir = format!("/home/{}/Documents/", user);
        println!("{right}");

        let mut n1 = CreateMode::new(vec![], left.clone(), left_dir);
        n1.run();
        let mut n2 = CreateMode::new(vec![], right.clone(), right_dir);
        n2.run();

        let cm = CompareMode::new(vec![], left.clone(), right.clone());
    }
}
