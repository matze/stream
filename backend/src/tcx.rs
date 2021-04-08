use anyhow::Result;
use chrono::prelude::*;
use std::io::Read;
use uom::si::f32::{Length, Time};

pub struct TrackPoint {
    pub time: chrono::DateTime<Utc>,
    pub heart_rate: i32,
    pub position: Option<geo::Point<f64>>,
    pub sensor_state: String,
}

pub struct Lap {
    pub total_time: Time,
    pub distance: Length,
    pub track_points: Vec<TrackPoint>,
}

pub struct Activity {
    pub sport: String,
    pub id: chrono::DateTime<Utc>,
    pub laps: Vec<Lap>,
}

pub struct Database {
    pub activities: Vec<Activity>,
}

pub mod xml {
    use anyhow::Result;
    use chrono::prelude::*;
    use serde::Deserialize;
    use std::io::Read;
    use uom::si::f32::{Length, Time};

    #[derive(Deserialize)]
    struct HeartRate {
        #[serde(rename = "Value")]
        value: i32,
    }

    #[derive(Deserialize)]
    struct Position {
        #[serde(rename = "LatitudeDegrees")]
        lat: f64,

        #[serde(rename = "LongitudeDegrees")]
        lon: f64,
    }

    #[derive(Deserialize)]
    struct Sample {
        #[serde(rename = "Time")]
        time: chrono::DateTime<Utc>,

        #[serde(rename = "Position")]
        position: Option<Position>,

        #[serde(rename = "HeartRateBpm")]
        heart_rate: HeartRate,

        #[serde(rename = "SensorState")]
        sensor_state: String,
    }

    #[derive(Deserialize)]
    struct Track {
        #[serde(rename = "Trackpoint")]
        samples: Vec<Sample>,
    }

    #[derive(Deserialize)]
    struct Lap {
        #[serde(rename = "TotalTimeSeconds")]
        total_time: Time,

        #[serde(rename = "DistanceMeters")]
        distance: Length,

        #[serde(rename = "Track")]
        track: Track,
    }

    #[derive(Deserialize)]
    struct Activity {
        #[serde(rename = "Sport")]
        sport: String,

        #[serde(rename = "Id")]
        id: chrono::DateTime<Utc>,

        #[serde(rename = "Lap")]
        laps: Vec<Lap>,
    }

    #[derive(Deserialize)]
    struct Activities {
        #[serde(rename = "Activity")]
        activity: Activity,
    }

    #[derive(Deserialize)]
    struct TrainingCenterDatabase {
        #[serde(rename = "Activities")]
        activities: Vec<Activities>,
    }

    impl super::TrackPoint {
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

    impl super::Lap {
        fn from(lap: Lap) -> Self {
            Self {
                total_time: lap.total_time,
                distance: lap.distance,
                track_points: lap
                    .track
                    .samples
                    .into_iter()
                    .map(|sample| super::TrackPoint::from(sample))
                    .collect(),
            }
        }
    }

    impl super::Activity {
        fn from(a: Activities) -> Self {
            Self {
                sport: a.activity.sport,
                id: a.activity.id,
                laps: a
                    .activity
                    .laps
                    .into_iter()
                    .map(|lap| super::Lap::from(lap))
                    .collect(),
            }
        }
    }

    pub fn from_reader<R: Read>(reader: R) -> Result<super::Database> {
        let db: TrainingCenterDatabase = serde_xml_rs::from_reader(reader)?;

        Ok(super::Database {
            activities: db
                .activities
                .into_iter()
                .map(|a| super::Activity::from(a))
                .collect(),
        })
    }
}

impl Database {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        Ok(xml::from_reader(reader)?)
    }
}
