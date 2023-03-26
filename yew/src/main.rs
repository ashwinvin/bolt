use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use stylist::StyleSource;
use tauri_sys::tauri;
use utils::*;
use yew::{Component, Context, Html};

mod html_sources;
mod style;
mod utils;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
}

pub struct BoltApp {
    style: StyleSource,

    method: Method,
    _request: String,
    _response: String,

    req_tab: u8,
    resp_tab: u8,

    req_body_tab_ref: yew::NodeRef,
    req_params_tab_ref: yew::NodeRef,
    req_headers_tab_ref: yew::NodeRef,

    resp_body_tab_ref: yew::NodeRef,
    resp_headers_tab_ref: yew::NodeRef,
}

impl Component for BoltApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: style::get_styles(),

            method: Method::GET,
            _request: "http:".to_string(),
            _response: "the response".to_string(),

            req_tab: 1,
            resp_tab: 1,

            req_body_tab_ref: yew::NodeRef::default(),
            req_params_tab_ref: yew::NodeRef::default(),
            req_headers_tab_ref: yew::NodeRef::default(),

            resp_body_tab_ref: yew::NodeRef::default(),
            resp_headers_tab_ref: yew::NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedMethod(meth) => {
                self.method = meth;

                return true;
            }

            Msg::SendPressed => {
                send_pressed(&get_url(), &get_method());

                return true;
            }

            Msg::ReqBodyPressed => {
                self.req_tab = 1;

                switch_req_tab(self, 1);
                return true;
            }

            Msg::ReqHeadersPressed => {
                self.req_tab = 3;

                switch_req_tab(self, 3);
                return true;
            }

            Msg::ReqParamsPressed => {
                self.req_tab = 2;

                switch_req_tab(self, 2);
                return true;
            }

            Msg::RespBodyPressed => {
                self.resp_tab = 1;

                switch_resp_tab(self, 1);
                return true;
            }

            Msg::RespHeadersPressed => {
                self.resp_tab = 2;

                switch_resp_tab(self, 2);
                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html_sources::home::get_main(self, ctx)
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

fn send_pressed(url: &str, method: &str) {
    send_request(url, method);
}

fn send_request(url: &str, method: &str) {
    #[derive(Serialize, Deserialize)]
    struct Payload<'a> {
        url: &'a str,
        method: &'a str,
    }

    let url = url.to_string();
    let method = method.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        let _resp: String = tauri::invoke(
            "send_request",
            &Payload {
                url: &url,
                method: &method,
            },
        )
        .await
        .unwrap();
    });
}
