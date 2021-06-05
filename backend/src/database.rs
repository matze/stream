use anyhow::{anyhow, Result};
use sha2_const::Sha256;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{read_dir, File};
use std::io::{BufReader, Read};
use std::path::Path;
use uom::si::f64::Length;
use uom::si::length::kilometer;

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
    pub activities: Vec<common::Activity>,
    pub laps: HashMap<Id, Vec<common::Lap>>,
}

fn read_activities<R: Read>(reader: R) -> Result<Vec<tcx::Activity>> {
    let db = tcx::Database::new(reader)?;
    Ok(db
        .activities
        .into_iter()
        .map(|a| a.activity)
        .collect::<Vec<_>>())
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut indexed_activities = vec![];
        let mut indexed_laps = HashMap::new();

        for filename in read_dir(path)? {
            let reader = BufReader::new(File::open(filename?.path())?);

            for activity in read_activities(reader)? {
                let new_id = Id::new(&activity.id.to_string());
                let tcx::Activity { sport, id: _, laps } = activity;

                let mut average_heart_rate: f64 = 0.0;
                let mut total_distance: Length = Length::new::<kilometer>(0.0);

                let laps = laps
                    .into_iter()
                    .map(|l| common::Lap::from(l))
                    .collect::<Vec<_>>();

                for lap in &laps {
                    average_heart_rate += lap
                        .track_points
                        .iter()
                        .map(|p| p.heart_rate as f64)
                        .sum::<f64>()
                        / (lap.track_points.len() as f64);

                    total_distance += lap.distance;
                }

                average_heart_rate /= laps.len() as f64;

                indexed_activities.push(common::Activity {
                    sport: common::Sport::from(sport),
                    id: String::from(&new_id),
                    average_heart_rate,
                    total_distance,
                });

                indexed_laps.insert(new_id, laps);
            }
        }

        Ok(Self {
            activities: indexed_activities,
            laps: indexed_laps,
        })
    }
}
