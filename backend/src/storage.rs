// use crate::tcx::Database;
use anyhow::Result;
use common::Activity;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufReader, Read};
use std::path::Path;

pub struct Database {
    pub activities: Vec<Activity>,
}

impl Database {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let db = tcx::TrainingCenterDatabase::from_reader(reader)?;

        Ok(Database {
            activities: db
                .activities
                .into_iter()
                .map(|a| Activity::from(a))
                .collect(),
        })
    }
}

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
