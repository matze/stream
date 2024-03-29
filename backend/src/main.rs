#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod database;

use anyhow::Result;
use database::Database;
use rocket::response::content::Html;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

#[get("/api/v1/activities")]
fn activities(db: State<Database>) -> Json<Vec<common::Activity>> {
    Json(db.activities.clone())
}

#[get("/api/v1/activity/<id>/laps")]
fn laps(id: String, db: State<Database>) -> Json<Vec<common::Lap>> {
    let hash = database::Id::from(&id).unwrap();
    Json(db.laps.get(&hash).unwrap().clone())
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>Stream</title>
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.7.1/dist/leaflet.css" integrity="sha512-xodZBNTC5n17Xt2atTPuE1HxjVMSvLVW9ocqUKLsCC5CXdbqCmblAshOMAS6/keqq/sMZMZ19scR4PsZChSR7A==" crossorigin=""/>
    <script src="https://unpkg.com/leaflet@1.7.1/dist/leaflet.js" integrity="sha512-XQoYMqMTK8LvdxXYG3nZ448hOEQiglfqkJs1NOQV44cWnUrBc8PkAOcXy20w0vlaXaVUearIOBhiXZ5V3ynxwA==" crossorigin=""></script>
    <script type="module">
      import init from "./static/wasm.js"
      init()
    </script>
    <style>
      #map {
        margin: auto;
        width: 100%;
        height: 600px;
      }
    </style>
  </head>
  <body>
  </body>
</html>
"#,
    )
}

fn main() -> Result<()> {
    rocket::ignite()
        .manage(Database::new("storage")?)
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, activities, laps])
        .launch();

    Ok(())
}
