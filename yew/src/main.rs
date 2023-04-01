use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use stylist::StyleSource;
use tauri_sys::tauri;
use yew::{html::Scope, Component, Context, Html};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

mod style;
mod process;
mod utils;
mod view;

// http://localhost:2000/ping

// TODO: request tabs

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

    AddHeader,
    RemoveHeader(usize),

    AddParam,
    RemoveParam(usize),

    ReceivedResponse,

    MethodChanged,
    UrlChanged,
    BodyChanged,
    HeaderChanged(usize),
    ParamChanged(usize),

    Update,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoltApp {
    style: StyleSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    status: u16,
    body: String,
    headers: Vec<Vec<String>>,
    time: u32,
    size: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct Request {
    url: String,
    body: String,
    headers: Vec<Vec<String>>,
    params: Vec<Vec<String>>,
    method: Method,
}

struct AppState {
    link: Option<Scope<BoltApp>>,

    method: Method,

    request: Request,
    response: Response,

    req_tab: u8,
    resp_tab: u8,
}

unsafe impl Sync for BoltApp {}

unsafe impl Send for BoltApp {}

unsafe impl Sync for AppState {}

unsafe impl Send for AppState {}

impl AppState {
    fn new() -> Self {
        Self {
            link: None,

            method: Method::GET,

            req_tab: 1,
            resp_tab: 1,

            request: Request {
                url: "".to_string(),
                body: "".to_string(),
                headers: vec![vec!["".to_string(), "".to_string()]],
                params: vec![vec!["".to_string(), "".to_string()]],
                method: Method::GET,
            },

            response: Response {
                status: 0,
                body: "Response body".to_string(),
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

    fn create(ctx: &Context<Self>) -> Self {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.link = Some(ctx.link().clone());

        Self {
            style: style::style::get_styles(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let render: bool = process::update::process(msg);

        return render;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        view::home::get_main(self, ctx)
    }
}

fn send_request(request: Request) {
    #[derive(Debug, Serialize, Deserialize)]
    struct Payload {
        url: String,
        method: Method,
        body: String,
        headers: Vec<Vec<String>>,
    }

    let payload = Payload {
        url: parse_url(request.url, request.params),
        method: request.method,
        body: request.body,
        headers: request.headers,
    };

    // bolt_log(&format!("{:?}", payload));

    wasm_bindgen_futures::spawn_local(async move {
        let _resp: String = tauri::invoke("send_request", &payload).await.unwrap();
    });
}

fn parse_url(url: String, params: Vec<Vec<String>>) -> String {
    let mut new_url = url;

    if params.len() > 0 && params[0][0] != "" {
        new_url.push_str("?");
    }

    for (i, param) in params.iter().enumerate() {
        if param[0] == "" || param[1] == "" {
            // bolt_panic("Param at index {i} has empty field");
            continue;
        }

        new_url.push_str(&param[0]);
        new_url.push_str("=");
        new_url.push_str(&param[1]);

        if i != params.len() - 1 {
            new_url.push_str("&");
        }
    }

    // bolt_log(&format!("url is: {new_url}"));
    return new_url;
}

pub fn receive_response(data: &str) {
    let mut state = GLOBAL_STATE.lock().unwrap();

    // bolt_log("received a response");

    let mut response: Response = serde_json::from_str(data).unwrap();

    // bolt_log(&format!("{:?}", response));

    // FIXME: render body as html
    response.body = highlight_body(&response.body);

    state.response = response;

    let link = state.link.as_ref().unwrap();

    link.send_message(Msg::Update);
}

// FIXME: remove background
fn highlight_body(body: &String) -> String {
    // Add syntax highlighting
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let syntax = syntax_set.find_syntax_by_extension("json").unwrap();

    let html = highlighted_html_for_string(
        body,
        &syntax_set,
        &syntax,
        &theme_set.themes["Solarized (dark)"],
    )
    .unwrap();

    return html;
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
