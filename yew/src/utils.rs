use serde::{Deserialize, Serialize};
use tauri_sys::tauri;
use wasm_bindgen::prelude::*;

pub fn bolt_log(log: &str) {
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

pub fn receive_response(data: &str) {
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

pub fn set_size(code: u64) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "size").unwrap();

    div.set_inner_html(&code.to_string());
}

pub fn set_time(code: u32) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "time").unwrap();

    div.set_inner_html(&code.to_string());
}

pub fn set_status(code: u16) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "status").unwrap();

    div.set_inner_html(&code.to_string());
}

pub fn set_resp_body(content: String) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "respbody").unwrap();

    let text_area = div.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();

    text_area.set_value(&content);
}

pub fn get_method() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "methodselect").unwrap();

    let select = div.dyn_into::<web_sys::HtmlSelectElement>().unwrap();

    let value = select.value();

    return value;
}

pub fn get_url() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "urlinput").unwrap();

    let input = div.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let value = input.value();

    return value;
}
