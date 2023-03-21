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
}

impl Component for BoltApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            method: Method::GET,
            _request: "http:".to_string(),
            _response: "the response".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedMethod(meth) => {
                self.method = meth;

                return true;
            }

            Msg::SendPressed => {
                bolt_log("send pressed");

                send_pressed(&get_url(), &get_method());

                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html_sources::home::get_main(self, ctx)
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
