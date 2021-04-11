use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uom::si::f32::{Length, Time};

#[derive(Serialize, Deserialize, Clone)]
pub struct Activities {
    /// Maps datetime Id to sport type
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackPoint {
    pub time: chrono::DateTime<Utc>,
    pub heart_rate: i32,
    pub position: Option<geo::Point<f64>>,
    pub sensor_state: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Lap {
    pub total_time: Time,
    pub distance: Length,
    pub track_points: Vec<TrackPoint>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Sport {
    Running,
    Biking,
    Other,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Activity {
    pub sport: Sport,
    pub id: chrono::DateTime<Utc>,
}

impl From<tcx::Sample> for TrackPoint {
    fn from(sample: tcx::Sample) -> Self {
        let position = sample.position.map(|p| geo::Point::new(p.lat, p.lon));

        Self {
            time: sample.time,
            heart_rate: sample.heart_rate.value,
            sensor_state: sample.sensor_state,
            position,
        }
    }
}

impl From<tcx::Lap> for Lap {
    fn from(lap: tcx::Lap) -> Self {
        Self {
            total_time: lap.total_time,
            distance: lap.distance,
            track_points: lap
                .track
                .samples
                .into_iter()
                .map(|sample| TrackPoint::from(sample))
                .collect(),
        }
    }
}

impl From<tcx::Sport> for Sport {
    fn from(sport: tcx::Sport) -> Self {
        match sport {
            tcx::Sport::Running => Sport::Running,
            tcx::Sport::Biking => Sport::Biking,
            tcx::Sport::Other => Sport::Other,
        }
    }
}
