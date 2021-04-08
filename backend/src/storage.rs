use crate::tcx::{Activity, Database};
use anyhow::Result;
use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::path::Path;

pub fn load<P: AsRef<Path>>(path: P) -> Result<HashMap<chrono::DateTime<Utc>, Activity>> {
    let mut result = HashMap::new();

    for filename in read_dir(path)? {
        let reader = BufReader::new(File::open(filename?.path())?);
        let db = Database::from_reader(reader)?;

        for activity in db.activities {
            result.insert(activity.id, activity);
        }
    }

    Ok(result)
}
