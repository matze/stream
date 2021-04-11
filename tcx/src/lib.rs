use anyhow::Result;
use chrono::prelude::*;
use serde::Deserialize;
use std::io::Read;
use uom::si::f32::{Length, Time};

#[derive(Deserialize)]
pub struct HeartRate {
    #[serde(rename = "Value")]
    pub value: i32,
}

#[derive(Deserialize)]
pub struct Position {
    #[serde(rename = "LatitudeDegrees")]
    pub lat: f64,

    #[serde(rename = "LongitudeDegrees")]
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct Sample {
    #[serde(rename = "Time")]
    pub time: chrono::DateTime<Utc>,

    #[serde(rename = "Position")]
    pub position: Option<Position>,

    #[serde(rename = "HeartRateBpm")]
    pub heart_rate: HeartRate,

    #[serde(rename = "SensorState")]
    pub sensor_state: String,
}

#[derive(Deserialize)]
pub struct Track {
    #[serde(rename = "Trackpoint")]
    pub samples: Vec<Sample>,
}

#[derive(Deserialize)]
pub struct Lap {
    #[serde(rename = "TotalTimeSeconds")]
    pub total_time: Time,

    #[serde(rename = "DistanceMeters")]
    pub distance: Length,

    #[serde(rename = "Track")]
    pub track: Track,
}

#[derive(Deserialize)]
pub enum Sport {
    Running,
    Biking,
    Other,
}

#[derive(Deserialize)]
pub struct Activity {
    #[serde(rename = "Sport")]
    pub sport: Sport,

    #[serde(rename = "Id")]
    pub id: chrono::DateTime<Utc>,

    #[serde(rename = "Lap")]
    pub laps: Vec<Lap>,
}

#[derive(Deserialize)]
pub struct Activities {
    #[serde(rename = "Activity")]
    pub activity: Activity,
}

#[derive(Deserialize)]
pub struct TrainingCenterDatabase {
    #[serde(rename = "Activities")]
    pub activities: Vec<Activities>,
}

impl TrainingCenterDatabase {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let db: TrainingCenterDatabase = serde_xml_rs::from_reader(reader)?;
        Ok(db)
    }
}
