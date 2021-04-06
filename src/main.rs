mod storage;
mod tcx;

use anyhow::Result;
use askama::Template;

struct Sample {
    position: geo::Point<f64>,
    heart_rate: i32,
}

#[derive(Template)]
#[template(path = "index.html")]
struct RenderTemplate {
    samples: Vec<Sample>,
}

impl Sample {
    fn from(track_point: tcx::TrackPoint) -> Option<Self> {
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

fn main() -> Result<()> {
    let activities = storage::load("storage")?;
    let activity = activities.into_iter().map(|a| a.1).nth(0).unwrap();
    let points = itertools::concat(activity.laps.into_iter().map(|l| l.track_points));
    let samples: Vec<Sample> = points.into_iter().filter_map(|p| Sample::from(p)).collect();
    let template = RenderTemplate { samples };

    println!("{}", template.render()?);

    Ok(())
}
