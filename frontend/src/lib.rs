use common::{Activity, Activities};
use leaflet::{LatLng, Map, TileLayer};
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::console::ConsoleService;

struct Model {
    link: ComponentLink<Self>,
    activities: Activities,
    fetch_task: Option<FetchTask>,
    map: Option<Map>,
}

enum Msg {
    Get(Activities),
    Select(String),
    SelectResponse(Activity),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get("/api/v1/activities").body(Nothing).unwrap();
        let callback = link.callback(
            |response: Response<Json<Result<Activities, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::Get(data.unwrap())
            },
        );

        let component = Self {
            link,
            activities: Activities {
                ids: vec![],
            },
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
                ConsoleService::info(&id);
                let request = Request::get(format!("/api/v1/activity/{}", id)).body(Nothing).unwrap();
                let callback = self.link.callback(
                    |response: Response<Json<Result<Activity, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::SelectResponse(data.unwrap())
                    },
                );

                // Can we re-use this?
                self.fetch_task = Some(FetchService::fetch(request, callback).unwrap());
            }
            Msg::SelectResponse(activity) => {
                ConsoleService::info(&format!("selected {}", activity.sport));
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
                    { for self.activities.ids.iter().map(|e| self.view_activity(e)) }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_activity(&self, id: &String) -> Html {
        let select_id = id.clone();

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
