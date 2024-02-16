use std::{hash::Hasher, io::Read};

use fxhash::FxHasher;

const CARGO_EMIT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum HashFileOutcome {
    Created,
    Changed,
    Unchanged,
}

/// Check if the contents of a file have changed using its hash instead of its modification time.
pub fn compare_and_set_contents_hash(path: &str) -> HashFileOutcome {
    let computed_contents_hash: u64 = {
        let mut hasher = FxHasher::default();
        let file =
            std::fs::File::open(path).expect(format!("failed to open file at '{}'", path).as_str());
        let mut reader = std::io::BufReader::new(file);
        hasher.write(CARGO_EMIT_VERSION.as_bytes());
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = reader.read(&mut buffer).expect("failed to read file");
            if bytes_read == 0 {
                break;
            }
            hasher.write(&buffer[..bytes_read]);
        }
        hasher.finish()
    };

    let path_hash: u64 = {
        let mut hasher = FxHasher::default();
        hasher.write(path.as_bytes());
        hasher.finish()
    };

    let hash_file_name = format!("{:x}.hash", path_hash);
    let out_dir = std::env::var("OUT_DIR")
        .expect("failed to get OUT_DIR. Are you using the function in a build script?");
    let hash_file_path = std::path::Path::new(&out_dir).join(hash_file_name);

    if !hash_file_path.exists() {
        std::fs::write(&hash_file_path, computed_contents_hash.to_ne_bytes())
            .expect("failed to write hash file");
        return HashFileOutcome::Created;
    };

    let stored_contents_hash = std::fs::read(&hash_file_path).expect("failed to read hash file");
    assert!(stored_contents_hash.len() == computed_contents_hash.to_ne_bytes().len());

    if stored_contents_hash == computed_contents_hash.to_ne_bytes() {
        HashFileOutcome::Unchanged
    } else {
        std::fs::write(&hash_file_path, computed_contents_hash.to_ne_bytes())
            .expect("failed to write hash file");
        HashFileOutcome::Changed
    }
}
