use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufReader, Read};
use std::path::Path;

pub struct Database {
    pub activities: HashMap<String, common::Activity>,
    pub laps: HashMap<String, Vec<common::Lap>>,
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
        let mut indexed_activities = HashMap::new();
        let mut indexed_laps = HashMap::new();

        for filename in read_dir(path)? {
            let reader = BufReader::new(File::open(filename?.path())?);

            for activity in read_activities(reader)? {
                let mut hasher = Sha256::new();
                hasher.update(activity.id.to_string().as_bytes());

                let tcx::Activity { sport, id, laps } = activity;

                let index = hex::encode(hasher.finalize());

                indexed_laps.insert(
                    index.clone(),
                    laps.into_iter()
                        .map(|l| common::Lap::from(l))
                        .collect::<Vec<_>>(),
                );

                indexed_activities.insert(
                    index,
                    common::Activity {
                        sport: common::Sport::from(sport),
                        id,
                    },
                );
            }
        }

        Ok(Self {
            activities: indexed_activities,
            laps: indexed_laps,
        })
    }
}
