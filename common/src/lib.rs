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
    pub laps: Vec<Lap>,
}
