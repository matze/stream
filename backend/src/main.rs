#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod storage;
mod tcx;

use anyhow::Result;
use chrono::prelude::*;
use rocket::response::content::Html;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;

type ActivityMap = HashMap<chrono::DateTime<Utc>, tcx::Activity>;

#[get("/api/v1/foo")]
fn foo() -> String {
    "foo".to_owned()
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
        .manage(storage::load("storage")?)
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, foo])
        .launch();

    Ok(())
}
