use Fasching::{create_snapshot, export_snapshot};
use Fasching::hasher::HashType::BLAKE3;
use Fasching::snapshot::Snapshot;
use crate::syscompare::Comparer;

pub struct CreateMode {
    path: String,
    args: Vec<String>,
    snapshot: Snapshot
}

impl CreateMode {
    pub fn new(args: Vec<String>, path: String) -> CreateMode {
        if path.replace("./", "").is_empty() {
            panic!("Specify output file name")
        }

        CreateMode { args, path, snapshot: Default::default() }
    }
}

impl Comparer for CreateMode {
    fn run(&mut self) {
        let snapshot = create_snapshot(self.path.as_str(), BLAKE3, vec![]);
        self.snapshot = snapshot.clone();
        println!("Total FileHash Entries {}", snapshot.file_hashes.lock().unwrap().len());
        let _ = export_snapshot(self.snapshot.clone(), self.path.clone());
    }
}
