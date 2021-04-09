use leaflet::{LatLng, Map, TileLayer};
use wasm_bindgen::prelude::*;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch;
use yew::services::ConsoleService;

struct Model {
    link: ComponentLink<Self>,
    value: String,
    fetch_task: Option<fetch::FetchTask>,
    map: Option<Map>,
}

enum Msg {
    AddOne,
    Get(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = fetch::Request::get("/api/v1/foo").body(Nothing).unwrap();
        let callback = link.callback(|response: fetch::Response<Result<String, anyhow::Error>>| {
            Msg::Get(response.into_body().unwrap())
        });

        let component = Self {
            link,
            value: "".to_owned(),
            fetch_task: Some(fetch::FetchService::fetch(request, callback).unwrap()),
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
            Msg::AddOne => {}
            Msg::Get(response) => {
                ConsoleService::info(&format!("Got {}", response));
                self.value = response;
                self.fetch_task = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        ConsoleService::info(&format!("here"));
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div id="map"></div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value.clone() }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
