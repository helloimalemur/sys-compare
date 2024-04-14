use anyhow::Error;
use filesystem_hashing::snapshot::{Snapshot, SnapshotChangeType, SnapshotCompareResult};
use filesystem_hashing::{compare_snapshots, import_snapshot};

#[derive(Debug)]
pub struct CompareMode {
    left: Snapshot,
    right: Snapshot,
    selection: Option<String>,
    count_only: Option<bool>,
    #[allow(unused)]
    result_type: SnapshotChangeType,
    results: SnapshotCompareResult,
}

impl CompareMode {
    pub fn new(
        left: String,
        right: String,
        selection: Option<String>,
        count_only: Option<bool>,
    ) -> CompareMode {
        let left = import_snapshot(left).unwrap_or_default();
        let right = import_snapshot(right).unwrap_or_default();

        CompareMode {
            left,
            right,
            selection,
            count_only,
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
    pub(crate) fn run(&mut self) -> Result<(), Error> {
        let selector = match &self.selection {
            None => "none",
            Some(r) => r.as_str(),
        };

        let results = match compare_snapshots(self.left.clone(), self.right.clone()) {
            Some(x) => x,
            None => panic!("Error Comparing Snapshot"),
        };
        self.results = results.1;
        self.result_type = results.0;

        macro_rules! print_if_not_empty {
            ($ret:expr, $co:expr) => {
                if let Some(count_only) = $co {
                    if count_only {
                        println!("{}", $ret.len());
                    } else {
                        $ret.iter().for_each(|e| println!("{e}"));
                        println!("Created: {:?}", $ret.len());
                    }
                } else {
                    $ret.iter().for_each(|e| println!("{e}"));
                    println!("Created: {:?}", $ret.len());
                }
            };
        }

        match selector {
            "created" => {
                print_if_not_empty!(self.results.created, self.count_only);
            }
            "deleted" => {
                print_if_not_empty!(self.results.deleted, self.count_only);
            }
            "changed" => {
                print_if_not_empty!(self.results.changed, self.count_only);
            }
            "none" => {
                println!("Created: {:?}", self.results.created.len());
                println!("Deleted: {:?}", self.results.deleted.len());
                println!("Changed: {:?}", self.results.changed.len());
            }
            _ => {
                println!("Created: {:?}", self.results.created.len());
                println!("Deleted: {:?}", self.results.deleted.len());
                println!("Changed: {:?}", self.results.changed.len());
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::comparemode::CompareMode;
    use crate::createmode::CreateMode;
    use crate::options::{Arguments, Commands};
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

        let mut n1 = CreateMode::new(left.clone(), "/etc".to_string());
        let _ = n1.run();
        let mut n2 = CreateMode::new(right.clone(), "/etc".to_string());
        let _ = n2.run();

        let cm = CompareMode::new(left, right, None, None);

        // println!("{:?}", cm);
        assert!(cm.left.file_hashes.lock().unwrap().len() > 0);
        assert!(cm.right.file_hashes.lock().unwrap().len() > 0);
        // assert!(cm.results. > 0);
        // todo()! finish writing tests
    }
}
