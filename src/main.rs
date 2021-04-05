mod tcx;

use anyhow::Result;
use askama::Template;
use std::fs::File;
use std::io::BufReader;

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
    let reader = BufReader::new(File::open("test.tcx")?);
    let db = tcx::Database::from_reader(reader)?;
    let laps: Vec<tcx::Lap> = itertools::concat(db.activities.into_iter().map(|a| a.laps));
    let points: Vec<tcx::TrackPoint> = itertools::concat(laps.into_iter().map(|l| l.track_points));
    let samples: Vec<Sample> = points.into_iter().filter_map(|p| Sample::from(p)).collect();
    let template = RenderTemplate { samples };

    println!("{}", template.render()?);

    Ok(())
}
