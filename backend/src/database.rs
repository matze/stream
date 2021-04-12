use anyhow::{anyhow, Result};
use sha2_const::Sha256;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{read_dir, File};
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Id {
    hash: [u8; 32],
}

impl Id {
    pub fn new(s: &str) -> Self {
        let hash = Sha256::new().update(s.as_bytes()).finalize();
        Self { hash }
    }

    pub fn from(s: &str) -> Result<Self> {
        let hash: [u8; 32] = hex::decode(s)?
            .try_into()
            .map_err(|_| anyhow!("Cannot convert {} to Id", s))?;

        Ok(Self { hash })
    }
}

impl From<&Id> for String {
    fn from(id: &Id) -> Self {
        hex::encode(id.hash)
    }
}

pub struct Database {
    pub activities: HashMap<Id, common::Activity>,
    pub laps: HashMap<Id, Vec<common::Lap>>,
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
                let new_id = Id::new(&activity.id.to_string());
                let tcx::Activity { sport, id, laps } = activity;

                indexed_laps.insert(
                    new_id.clone(),
                    laps.into_iter()
                        .map(|l| common::Lap::from(l))
                        .collect::<Vec<_>>(),
                );

                indexed_activities.insert(
                    new_id,
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
