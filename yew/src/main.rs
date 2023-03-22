use wasm_bindgen::prelude::*;
use yew::{Component, Context, Html};

mod html_sources;
mod net;

#[wasm_bindgen(module = "/script.js")]
extern "C" {
    fn send_request(url: &str, method: &str);

    fn get_method() -> String;

    fn get_url() -> String;

    fn set_respbody(content: &str);

    fn bolt_log(log: &str);
}

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
    yew::Renderer::<BoltApp>::new().render();
}

#[wasm_bindgen]
pub fn yew_func() {
    bolt_log("yew was called!!");
}

fn send_pressed(url: &str, method: &str) {
    send_request(url, method);
}
