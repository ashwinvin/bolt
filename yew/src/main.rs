use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use stylist::StyleSource;
use tauri_sys::tauri;
use utils::*;
use yew::{Component, Context, Html};

mod html_sources;
mod style;
mod utils;

// http://localhost:2000/ping

// #[wasm_bindgen(module = "/script.js")]
// extern "C" {}

// Define the possible messages which can be sent to the component
pub enum Msg {
    SelectedMethod(Method),
    SendPressed,
    ReqBodyPressed,
    ReqHeadersPressed,
    ReqParamsPressed,

    RespBodyPressed,
    RespHeadersPressed,

    ReceivedResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoltApp {
    style: StyleSource,
}

#[derive(Clone, Serialize, Deserialize)]
struct HttpResponse {
    status: u16,
    body: String,
    headers: Vec<Vec<String>>,
    time: u32,
    size: u64,
}

#[derive(Serialize)]
struct AppState {
    // ctx: Option<BoltApp>,
    method: Method,
    count: i32,
    response: HttpResponse,

    req_tab: u8,
    resp_tab: u8,
}

impl AppState {
    fn new() -> Self {
        Self {
            // ctx: None,
            method: Method::GET,

            req_tab: 1,
            resp_tab: 1,
            count: 0,
            response: HttpResponse {
                status: 0,
                body: "my response".to_string(),
                headers: Vec::new(),
                time: 0,
                size: 0,
            },
        }
    }
}

// Create a shared global state variable
lazy_static::lazy_static! {
    static ref GLOBAL_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new()));
}

impl Component for BoltApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: style::get_styles(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut state = GLOBAL_STATE.lock().unwrap();

        match msg {
            Msg::SelectedMethod(meth) => {
                state.method = meth;

                return true;
            }

            Msg::SendPressed => {
                send_pressed();

                return true;
            }

            Msg::ReqBodyPressed => {
                state.req_tab = 1;

                switch_req_tab(1);
                return true;
            }

            Msg::ReqHeadersPressed => {
                state.req_tab = 3;

                switch_req_tab(3);
                return true;
            }

            Msg::ReqParamsPressed => {
                state.req_tab = 2;

                switch_req_tab(2);
                return true;
            }

            Msg::RespBodyPressed => {
                state.resp_tab = 1;

                switch_resp_tab(1);
                return true;
            }

            Msg::RespHeadersPressed => {
                state.resp_tab = 2;

                switch_resp_tab(2);
                return true;
            }

            Msg::ReceivedResponse => {
                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html_sources::home::get_main(self, ctx)
    }
}

impl BoltApp {
    fn render_header(&self, key: &String, value: &String) -> Html {
        yew::html! {
            <tr>
                <td>{key}</td>
                <td>{value}</td>
            </tr>
        }
    }
}

fn main() {
    wasm_bindgen_futures::spawn_local(async move {
        let mut events = tauri_sys::event::listen::<String>("receive_response")
            .await
            .expect("could not create response listener");

        while let Some(event) = events.next().await {
            receive_response(&event.payload);
        }
    });

    yew::Renderer::<BoltApp>::new().render();
}

fn send_pressed() {
    send_request(get_url(), get_method(), get_body(), get_headers());
}

fn send_request(url: String, method: String, body: String, headers: Vec<Vec<String>>) {
    #[derive(Debug, Serialize, Deserialize)]
    struct Payload {
        url: String,
        method: String,
        body: String,
        headers: Vec<Vec<String>>,
    }

    let payload = Payload {
        url,
        method,
        body,
        headers,
    };

    bolt_log(&format!("{:?}", payload));
    
    wasm_bindgen_futures::spawn_local(async move {
        let _resp: String = tauri::invoke("send_request", &payload).await.unwrap();
    });
}

pub fn receive_response(data: &str) {
    let mut state = GLOBAL_STATE.lock().unwrap();

    bolt_log("received a response");

    let response: HttpResponse = serde_json::from_str(data).unwrap();

    // let agent = self.link.agent();

    state.response = response.clone();

    set_resp_body(response.body);
    set_status(response.status);
    set_time(response.time);
    set_size(response.size);
}
