use Fasching::{create_snapshot, export_snapshot};
use Fasching::hasher::HashType::BLAKE3;
use Fasching::snapshot::Snapshot;
use crate::syscompare::Comparer;

pub struct CreateMode {
    in_path: String,
    out_path: String,
    args: Vec<String>,
    snapshot: Snapshot
}

impl CreateMode {
    pub fn new(args: Vec<String>, in_path: String, out_path: String) -> CreateMode {
        if out_path.replace("./", "").is_empty() {
            panic!("Specify output file name")
        }

        CreateMode { in_path, out_path, args, snapshot: Default::default() }
    }
}

impl Comparer for CreateMode {
    fn run(&mut self) {
        let snapshot = create_snapshot(self.in_path.as_str(), BLAKE3);
        println!("Total FileHash Entries {}", snapshot.file_hashes.lock().unwrap().len());
        let _ = export_snapshot(snapshot, self.out_path.clone());
    }
}
