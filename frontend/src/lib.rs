use common::{Activity, Lap};
use leaflet::{LatLng, Map, Polyline, TileLayer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Serialize, Deserialize)]
struct PolylineOptions {
    color: String,
}

struct Model {
    link: ComponentLink<Self>,
    activities: Vec<Activity>,
    fetch_task: Option<FetchTask>,
    map: Option<Map>,
}

enum Msg {
    Get(Vec<Activity>),
    Select(String),
    SelectResponse(Vec<Lap>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get("/api/v1/activities").body(Nothing).unwrap();
        let callback = link.callback(
            |response: Response<Json<Result<Vec<Activity>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::Get(data.unwrap())
            },
        );

        let component = Self {
            link,
            activities: vec![],
            fetch_task: Some(FetchService::fetch(request, callback).unwrap()),
            map: None,
        };

        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.map.is_none() {
            let map = Map::new("map", &JsValue::NULL);
            map.setView(&LatLng::new(49.0, 8.4), 14.0);

            TileLayer::new(
                "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
                &JsValue::NULL,
            )
            .addTo(&map);

            self.map = Some(map);
        }

        match msg {
            Msg::Get(activities) => {
                self.activities = activities;
                self.fetch_task = None;
            }
            Msg::Select(id) => {
                let request = Request::get(format!("/api/v1/activity/{}/laps", id))
                    .body(Nothing)
                    .unwrap();
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<Lap>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::SelectResponse(data.unwrap())
                    },
                );

                // Can we re-use this?
                self.fetch_task = Some(FetchService::fetch(request, callback).unwrap());
            }
            Msg::SelectResponse(laps) => {
                if let Some(map) = &self.map {
                    for lap in laps {
                        let points = lap
                            .track_points
                            .iter()
                            .filter_map(|p| p.position)
                            // 🤨 ...
                            .map(|p| LatLng::new(p.lng(), p.lat()))
                            .collect::<Vec<_>>();

                        Polyline::new_with_options(
                            points.iter().map(JsValue::from).collect(),
                            &JsValue::from_serde(&PolylineOptions {
                                color: "blue".to_string(),
                            })
                            .unwrap(),
                        )
                        .addTo(map);
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div id="map"></div>
                <ul>
                    { for self.activities.iter().map(|a| self.view_activity(a)) }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_activity(&self, activity: &Activity) -> Html {
        let id = activity.id.clone();
        let select_id = activity.id.clone();

        html! {
            <li>
            <label onclick=self.link.callback(move |_| Msg::Select(select_id.clone()))>{ id }</label>
            </li>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
