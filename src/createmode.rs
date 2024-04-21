use anyhow::Error;
use filesystem_hashing::hasher::HashType::BLAKE3;
use filesystem_hashing::snapshot::Snapshot;
use filesystem_hashing::{create_snapshot, export_snapshot};
use std::process::exit;
use std::sync::{Arc, Mutex};

pub struct CreateMode {
    snapshot_path: String,
    root_path: String,
    snapshot: Snapshot,
    verbose: bool,
}

impl CreateMode {
    pub fn new(snapshot_path: String, root_path: String, verbose: Option<bool>) -> CreateMode {
        let mut verbosity = false;
        match verbose {
            Some(_v) => {
                verbosity = true
            },
            None => {
                verbosity = false
            }
        }

        if snapshot_path.replace("./", "").is_empty() {
            println!("Specify output file name");
            exit(0);
        }
        let bind = root_path.clone();
        let _rp = bind.as_str();

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
            verbose: verbosity,
        }
    }
}

impl CreateMode {
    pub fn run(&mut self, verbose: Option<bool>) -> Result<(), Error> {
        let mut verbosity = false;
        match verbose {
            Some(v) => {
                verbosity = v
            },
            None => {
                verbosity = true
            }
        }
        let snapshot = create_snapshot(
            self.root_path.as_str(),
            BLAKE3,
            vec![
                "/dev".to_string(),
                "/proc".to_string(),
                "/tmp".to_string(),
                "/sys".to_string(),
            ],
            verbosity
        )?;
        self.snapshot = snapshot.clone();
        if let Ok(e) = snapshot.file_hashes.lock() {
            println!("Total FileHash Entries {}", e.len());
        }
        export_snapshot(self.snapshot.clone(), self.snapshot_path.clone(), true, self.verbose)?;
        Ok(())
    }
}
