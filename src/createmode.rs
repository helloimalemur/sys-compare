use std::process::exit;
use std::sync::{Arc, Mutex};
use anyhow::Error;
use filesystem_hashing::hasher::HashType::BLAKE3;
use filesystem_hashing::snapshot::Snapshot;
use filesystem_hashing::{create_snapshot, export_snapshot};

pub struct CreateMode {
    snapshot_path: String,
    root_path: String,
    snapshot: Snapshot,
}

impl CreateMode {
    pub fn new(snapshot_path: String, root_path: String) -> CreateMode {
        if snapshot_path.replace("./", "").is_empty() {
            println!("Specify output file name");
            exit(0);
        }
        let bind = root_path.clone();
        let rp = bind.as_str();

        CreateMode {
            snapshot_path,
            root_path,
            snapshot: Snapshot {
                file_hashes: Arc::new(Mutex::new(Default::default())),
                black_list: vec![],
                root_path: "".to_string(),
                hash_type: BLAKE3,
                uuid: "".to_string(),
                date_created: 0,
            },
        }
    }
}

impl CreateMode {
    pub fn run(&mut self) -> Result<(), Error> {
        let snapshot = create_snapshot(self.root_path.as_str(), BLAKE3, vec![])?;
        self.snapshot = snapshot.clone();
        if let Ok(e) = snapshot.file_hashes.lock() {
            println!("Total FileHash Entries {}", e.len());
        }
        export_snapshot(self.snapshot.clone(), self.snapshot_path.clone(), true)?;
        Ok(())
    }
}
