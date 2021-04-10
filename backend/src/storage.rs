use crate::tcx::{Activity, Database};
use anyhow::Result;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::path::Path;

/// Map hashed activity identifier to activity
pub type Map = HashMap<String, Activity>;

pub fn load<P: AsRef<Path>>(path: P) -> Result<Map> {
    let mut result = HashMap::new();

    for filename in read_dir(path)? {
        let reader = BufReader::new(File::open(filename?.path())?);
        let db = Database::from_reader(reader)?;

        for activity in db.activities {
            let mut hasher = Sha256::new();
            hasher.update(activity.id.to_string().as_bytes());

            result.insert(hex::encode(hasher.finalize()), activity);
        }
    }

    Ok(result)
}
