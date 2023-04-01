use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use stylist::StyleSource;
use tauri_sys::tauri;
use utils::*;
use yew::{Component, Context, html::Scope, Html};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

mod html_sources;
mod style;
mod utils;

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

    Update
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
            style: style::get_styles(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedMethod(meth) => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.method = meth;

                return true;
            }

            Msg::SendPressed => {
                let state = GLOBAL_STATE.lock().unwrap();
                send_request(state.request.clone());

                return true;
            }

            Msg::ReqBodyPressed => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.req_tab = 1;

                switch_req_tab(1);
                return true;
            }

            Msg::ReqHeadersPressed => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.req_tab = 3;

                switch_req_tab(3);
                return true;
            }

            Msg::ReqParamsPressed => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.req_tab = 2;

                switch_req_tab(2);
                return true;
            }

            Msg::RespBodyPressed => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.resp_tab = 1;

                switch_resp_tab(1);
                return true;
            }

            Msg::RespHeadersPressed => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.resp_tab = 2;

                switch_resp_tab(2);
                return true;
            }

            Msg::ReceivedResponse => {
                return true;
            }

            Msg::AddHeader => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state
                    .request
                    .headers
                    .push(vec!["".to_string(), "".to_string()]);

                return true;
            }

            Msg::RemoveHeader(index) => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.request.headers.remove(index);

                return true;
            }

            Msg::AddParam => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state
                    .request
                    .params
                    .push(vec!["".to_string(), "".to_string()]);

                return true;
            }

            Msg::RemoveParam(index) => {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.request.params.remove(index);

                return true;
            }

            Msg::MethodChanged => {
                let method = get_method();

                let mut state = GLOBAL_STATE.lock().unwrap();
                state.request.method = method;

                return true;
            }

            Msg::UrlChanged => {
                let url = get_url();

                let mut state = GLOBAL_STATE.lock().unwrap();
                state.request.url = url;

                return true;
            }

            Msg::BodyChanged => {
                let body = get_body();

                let mut state = GLOBAL_STATE.lock().unwrap();
                state.request.body = body;

                return true;
            }

            Msg::HeaderChanged(index) => {
                let header = get_header(index);

                let mut state = GLOBAL_STATE.lock().unwrap();
                state.request.headers[index] = header;

                return true;
            }

            Msg::ParamChanged(index) => {
                let param = get_param(index);

                let mut state = GLOBAL_STATE.lock().unwrap();
                state.request.params[index] = param;

                return true;
            }

            Msg::Update => {

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

    fn render_reqheader(
        &self,
        ctx: &Context<BoltApp>,
        index: usize,
        length: usize,
        key: &String,
        value: &String,
    ) -> Html {
        yew::html! {
            <tr>
                <td><input id={"headerkey".to_string() + &index.to_string()} type="text" class="tableinput" value={key.to_string()} onchange={ctx.link().callback(move |_| Msg::HeaderChanged(index))}/></td>
                <td class="tableline">
                    <input id={"headervalue".to_string() + &index.to_string()} type="text" class="tableinput" value={value.to_string()} onchange={ctx.link().callback(move |_| Msg::HeaderChanged(index))}/>
                    if index == length - 1 {
                        <div class="pointer" onclick={ctx.link().callback(|_| Msg::AddHeader)}>
                            <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                        </div>
                    }else {
                        <div class="pointer" onclick={ctx.link().callback(move |_| Msg::RemoveHeader(index.clone()))}>
                            <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
                        </div>
                    }
                </td>
            </tr>
        }
    }

    fn render_params(
        &self,
        ctx: &Context<BoltApp>,
        index: usize,
        length: usize,
        key: &String,
        value: &String,
    ) -> Html {
        yew::html! {
            <tr>
                <td><input id={"paramkey".to_string() + &index.to_string()} type="text" class="tableinput" value={key.to_string()} onchange={ctx.link().callback(move |_| Msg::ParamChanged(index))}/></td>
                <td class="tableline">
                    <input id={"paramvalue".to_string() + &index.to_string()} type="text" class="tableinput" value={value.to_string()} onchange={ctx.link().callback(move |_| Msg::ParamChanged(index))}/>
                    if index == length - 1 {
                        <div class="pointer" onclick={ctx.link().callback(|_| Msg::AddParam)}>
                            <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                        </div>
                    }else {
                        <div class="pointer" onclick={ctx.link().callback(move |_| Msg::RemoveParam(index.clone()))}>
                            <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
                        </div>
                    }
                </td>
            </tr>
        }
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

    bolt_log(&format!("{:?}", payload));

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

    bolt_log("received a response");

    let mut response: Response = serde_json::from_str(data).unwrap();

    bolt_log(&format!("{:?}", response));

    // FIXME: render body as html
    response.body = highlight_body(&response.body);

    state.response = response;

    // TODO: trigger message after receiving response
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
