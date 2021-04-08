#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod storage;
mod tcx;

use anyhow::Result;
use askama::Template;
use chrono::prelude::*;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

type ActivityMap = HashMap<chrono::DateTime<Utc>, tcx::Activity>;

#[get("/api/v1/foo")]
fn foo() -> String {
    "foo".to_owned()
}

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {}
}

fn main() -> Result<()> {
    rocket::ignite()
        .manage(storage::load("storage")?)
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, foo])
        .launch();

    Ok(())
}
