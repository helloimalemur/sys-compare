use Fasching::{create_snapshot, export_snapshot};
use Fasching::hasher::HashType::BLAKE3;
use Fasching::snapshot::Snapshot;
use crate::syscompare::Comparer;

pub struct CreateMode {
    snapshot_path: String,
    root_path: String,
    args: Vec<String>,
    snapshot: Snapshot
}

impl CreateMode {
    pub fn new(args: Vec<String>, snapshot_path: String, root_path: String) -> CreateMode {
        if snapshot_path.replace("./", "").is_empty() {
            panic!("Specify output file name")
        }

        CreateMode { args, snapshot_path, root_path, snapshot: Default::default() }
    }
}

impl Comparer for CreateMode {
    fn run(&mut self) {
        let snapshot = create_snapshot(self.root_path.as_str(), BLAKE3, vec![]);
        self.snapshot = snapshot.clone();
        println!("Total FileHash Entries {}", snapshot.file_hashes.lock().unwrap().len());
        let _ = export_snapshot(self.snapshot.clone(), self.snapshot_path.clone());
    }
}
