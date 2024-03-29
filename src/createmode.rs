use crate::print_help;
use crate::syscompare::Comparer;
use std::process::exit;
use Fasching::hasher::HashType;
use Fasching::hasher::HashType::BLAKE3;
use Fasching::snapshot::Snapshot;
use Fasching::{create_snapshot, export_snapshot};

pub struct CreateMode {
    snapshot_path: String,
    root_path: String,
    #[allow(unused)]
    args: Vec<String>,
    snapshot: Snapshot,
}

impl CreateMode {
    pub fn new(args: Vec<String>, snapshot_path: String, root_path: String) -> CreateMode {
        if snapshot_path.replace("./", "").is_empty() {
            println!("Specify output file name");
            print_help();
            exit(0);
        }
        let bind = root_path.clone();
        let rp = bind.as_str();
        CreateMode {
            args,
            snapshot_path,
            root_path,
            snapshot: create_snapshot(rp, HashType::MD5, vec![]),
        }
    }
}

impl Comparer for CreateMode {
    fn run(&mut self) {
        let snapshot = create_snapshot(self.root_path.as_str(), BLAKE3, vec![]);
        self.snapshot = snapshot.clone();
        if let Ok(e) = snapshot.file_hashes.lock() {
            println!("Total FileHash Entries {}", e.len());
        }
        export_snapshot(self.snapshot.clone(), self.snapshot_path.clone(), true);
    }
}
