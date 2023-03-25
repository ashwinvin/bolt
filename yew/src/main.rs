use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use tauri_sys::tauri;
use wasm_bindgen::prelude::*;
use yew::{Component, Context, Html};

mod html_sources;
mod net;

#[wasm_bindgen(module = "/script.js")]
extern "C" {}

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

fn switch_req_tab(sel: &BoltApp, index: u8) {
    match index {
        1 => {
            if let Some(div) = sel.req_body_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().add_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.req_params_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.req_headers_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }
        }

        2 => {
            if let Some(div) = sel.req_body_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.req_params_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().add_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.req_headers_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }
        }

        3 => {
            if let Some(div) = sel.req_body_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.req_params_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.req_headers_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().add_1("tabSelected").unwrap();
            }
        }

        _ => {}
    }
}

fn switch_resp_tab(sel: &BoltApp, index: u8) {
    match index {
        1 => {
            if let Some(div) = sel.resp_body_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().add_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.resp_headers_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }
        }

        2 => {
            if let Some(div) = sel.resp_body_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().remove_1("tabSelected").unwrap();
            }

            if let Some(div) = sel.resp_headers_tab_ref.cast::<web_sys::HtmlElement>() {
                div.class_list().add_1("tabSelected").unwrap();
            }
        }

        _ => {}
    }
}

fn main() {
    wasm_bindgen_futures::spawn_local(async move {
        let mut events = tauri_sys::event::listen::<String>("receive_response")
            .await
            .expect("could not create response listener");

        while let Some(event) = events.next().await {
            bolt_log("yew has gotten event");

            bolt_log(&format!("{}", &event.payload));

            receive_response(&event.payload);
        }
    });

    yew::Renderer::<BoltApp>::new().render();
}

#[wasm_bindgen]
pub fn yew_func() {
    bolt_log("yew was called!!");
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
        let new_msg: String = tauri::invoke(
            "send_request",
            &Payload {
                url: &url,
                method: &method,
            },
        )
        .await
        .unwrap();

        println!("{}", new_msg);
    });
}

fn bolt_log(log: &str) {
    #[derive(Serialize, Deserialize)]
    struct Payload<'a> {
        log: &'a str,
    }

    let log = log.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        let new_msg: String = tauri::invoke("bolt_log", &Payload { log: &log })
            .await
            .unwrap();

        println!("{}", new_msg);
    });
}

fn receive_response(data: &str) {
    bolt_log("received a response");

    #[derive(Serialize, Deserialize)]
    struct Payload {
        status: u16,
        body: String,
        time: u32,
        size: u64,
    }

    let response: Payload = serde_json::from_str(data).unwrap();

    set_resp_body(response.body);
    set_status(response.status);
    set_time(response.time);
    set_size(response.size);
}

fn set_size(code: u64) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "size").unwrap();

    div.set_inner_html(&code.to_string());
}

fn set_time(code: u32) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "time").unwrap();

    div.set_inner_html(&code.to_string());
}

fn set_status(code: u16) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "status").unwrap();

    div.set_inner_html(&code.to_string());
}

fn set_resp_body(content: String) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "respbody").unwrap();

    let text_area = div.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();

    text_area.set_value(&content);
}

fn get_method() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "methodselect").unwrap();

    let select = div.dyn_into::<web_sys::HtmlSelectElement>().unwrap();

    let value = select.value();

    return value;
}

fn get_url() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "urlinput").unwrap();

    let input = div.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let value = input.value();

    return value;
}
