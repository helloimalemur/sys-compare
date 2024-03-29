use Fasching::snapshot::{Snapshot, SnapshotChangeType, SnapshotCompareResult};
use Fasching::{compare_snapshots, import_snapshot};
use crate::options::Arguments;

pub struct CompareMode {
    left: Snapshot,
    right: Snapshot,
    selection: Option<String>,
    options: Arguments,
    result_type: SnapshotChangeType,
    results: SnapshotCompareResult,
}

impl CompareMode {
    pub fn new(options: Arguments, left: String, right: String, selection: Option<String>) -> CompareMode {
        let left = import_snapshot(left);
        let right = import_snapshot(right);

        CompareMode {
            left,
            right,
            selection,
            options,
            result_type: SnapshotChangeType::None,
            results: SnapshotCompareResult {
                created: vec![],
                deleted: vec![],
                changed: vec![],
            },
        }
    }
}

impl CompareMode {
    pub(crate) fn run(&mut self) {
        let selector = match &self.selection {
            None => "none",
            Some(r) => {
                r.as_str()
            },
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

        let mut n1 = CreateMode::new(vec![], left.clone());
        n1.run();
        let mut n2 = CreateMode::new(vec![], right.clone());
        n2.run();

        let cm = CompareMode::new(vec![], left.clone(), right.clone(), );
    }
}
