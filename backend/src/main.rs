#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod storage;
mod tcx;

use anyhow::Result;
use askama::Template;
use chrono::prelude::*;
use rocket::State;
use std::collections::HashMap;

struct Sample {
    position: geo::Point<f64>,
    heart_rate: i32,
}

impl Sample {
    fn from(track_point: &tcx::TrackPoint) -> Option<Self> {
        if let Some(position) = track_point.position {
            Some(Self {
                position: position,
                heart_rate: track_point.heart_rate,
            })
        } else {
            None
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct RenderTemplate {
    samples: Vec<Sample>,
}

type ActivityMap = HashMap<chrono::DateTime<Utc>, tcx::Activity>;

#[get("/")]
fn index(activities: State<ActivityMap>) -> RenderTemplate {
    let activity = activities.iter().map(|a| a.1).nth(0).unwrap();

    let mut samples = Vec::new();

    for lap in &activity.laps {
        for point in &lap.track_points {
            if let Some(sample) = Sample::from(&point) {
                samples.push(sample);
            }
        }
    }

    RenderTemplate { samples }
}

fn main() -> Result<()> {
    rocket::ignite()
        .manage(storage::load("storage")?)
        .mount("/", routes![index])
        .launch();

    Ok(())
}
