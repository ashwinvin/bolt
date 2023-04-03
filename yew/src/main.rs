use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use stylist::StyleSource;
use tauri_sys::tauri;
use yew::{html::Scope, Component, Context, Html};

use syntect::highlighting::ThemeSet;
use syntect::highlighting::{Color, Theme};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

mod process;
mod style;
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

    AddRequest,
    RemoveRequest(usize),
    SelectRequest(usize),

    Update,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    TEXT,
    JSON,
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Method {
    GET,
    POST,
}

impl Method {
    fn to_string(&self) -> String {
        match self {
            Method::GET => "get".to_string(),
            Method::POST => "post".to_string(),
        }
    }
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
    response_type: ResponseType,
}

impl Response {
    fn new() -> Self {
        Response {
            status: 0,
            body: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: ResponseType::TEXT,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Request {
    url: String,
    body: String,
    headers: Vec<Vec<String>>,
    params: Vec<Vec<String>>,
    method: Method,

    response: Response,

    // META
    name: String,
}

impl Request {
    fn new() -> Request {
        Request {
            url: String::new(),
            body: String::new(),
            headers: vec![vec![String::new(), String::new()]],
            params: vec![vec![String::new(), String::new()]],
            method: Method::GET,

            response: Response::new(),

            // META
            name: "NEW REQUEST ".to_string(),
        }
    }
}

struct AppState {
    link: Option<Scope<BoltApp>>,

    current_request: usize,

    requests: Vec<Request>,

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

            req_tab: 1,
            resp_tab: 1,

            current_request: 0,

            requests: vec![Request::new()],
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

    if response.response_type == ResponseType::JSON {
        response.body = format_json(&response.body);
        response.body = highlight_body(&response.body);
    }

    let current = state.current_request;
    state.requests[current].response = response;

    let link = state.link.as_ref().unwrap();

    link.send_message(Msg::Update);
}

fn format_json(data: &String) -> String {
    let value: serde_json::Value = serde_json::from_str(data).unwrap();

    let pretty = serde_json::to_string_pretty(&value).unwrap();

    return pretty;
}

fn create_custom_theme() -> Theme {
    let mut theme = ThemeSet::load_defaults().themes["Solarized (dark)"].clone();

    // Change the background color
    theme.settings.background = Some(Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    });

    theme
}

fn highlight_body(body: &String) -> String {
    // Add syntax highlighting
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme = create_custom_theme();
    let syntax = syntax_set.find_syntax_by_extension("json").unwrap();

    let html = highlighted_html_for_string(body, &syntax_set, &syntax, &theme).unwrap();

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
