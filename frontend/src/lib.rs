use wasm_bindgen::prelude::*;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch;
use yew::services::ConsoleService;

struct Model {
    link: ComponentLink<Self>,
    value: String,
    fetch_task: Option<fetch::FetchTask>,
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
        let callback = link.callback(
            |response: fetch::Response<Result<String, anyhow::Error>>| {
                Msg::Get(response.into_body().unwrap())
            },
        );

        let component = Self {
            link,
            value: "".to_owned(),
            fetch_task: Some(fetch::FetchService::fetch(request, callback).unwrap()),
        };

        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {},
            Msg::Get(response) => {
                ConsoleService::info(&format!("Got {}", response));
                self.value = response;
                self.fetch_task = None;
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
                <button class="#foo" onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value.clone() }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
