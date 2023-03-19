use wasm_bindgen::prelude::*;
use yew::{Component, Context, Html};

mod html_sources;
mod net;

#[wasm_bindgen(module = "/script.js")]
extern "C" {
    fn send_btn();

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
    _value: i64,
    method: Method,
    _request: String,
    response: String,
}

impl Component for BoltApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _value: 0,
            method: Method::GET,
            _request: "http:".to_string(),
            response: "the response".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedMethod(meth) => {
                self.method = meth;

                return true;
            }

            Msg::SendPressed => {
                bolt_log("yew is sending to tauri");
                send_btn();

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
