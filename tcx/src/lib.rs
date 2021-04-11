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

impl From<Sample> for common::TrackPoint {
    fn from(sample: Sample) -> Self {
        let position = sample.position.map(|p| geo::Point::new(p.lat, p.lon));

        Self {
            time: sample.time,
            heart_rate: sample.heart_rate.value,
            sensor_state: sample.sensor_state,
            position,
        }
    }
}

impl From<Lap> for common::Lap {
    fn from(lap: Lap) -> Self {
        Self {
            total_time: lap.total_time,
            distance: lap.distance,
            track_points: lap
                .track
                .samples
                .into_iter()
                .map(|sample| common::TrackPoint::from(sample))
                .collect(),
        }
    }
}

impl From<Sport> for common::Sport {
    fn from(sport: Sport) -> Self {
        match sport {
            Sport::Running => common::Sport::Running,
            Sport::Biking => common::Sport::Biking,
            Sport::Other => common::Sport::Other,
        }
    }
}

impl From<Activities> for common::Activity {
    fn from(a: Activities) -> Self {
        Self {
            sport: common::Sport::from(a.activity.sport),
            id: a.activity.id,
            laps: a
                .activity
                .laps
                .into_iter()
                .map(|lap| common::Lap::from(lap))
                .collect(),
        }
    }
}
