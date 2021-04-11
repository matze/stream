use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufReader, Read};
use std::path::Path;

pub struct Database {
    pub activities: HashMap<String, common::Activity>,
}

fn read_activities<R: Read>(reader: R) -> Result<Vec<tcx::Activity>> {
    let db = tcx::TrainingCenterDatabase::from_reader(reader)?;
    Ok(db
        .activities
        .into_iter()
        .map(|a| a.activity)
        .collect::<Vec<_>>())
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut activities = HashMap::new();

        for filename in read_dir(path)? {
            let reader = BufReader::new(File::open(filename?.path())?);

            for activity in read_activities(reader)? {
                let mut hasher = Sha256::new();
                hasher.update(activity.id.to_string().as_bytes());
                activities.insert(hex::encode(hasher.finalize()), common::Activity::from(activity));
            }
        }

        Ok(Self { activities })
    }
}
