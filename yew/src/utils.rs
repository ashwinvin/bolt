use crate::Method;
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
        let _resp: String = tauri::invoke("bolt_log", &Payload { log: &log })
            .await
            .unwrap();
    });
}

pub fn bolt_panic(log: &str) {
    #[derive(Serialize, Deserialize)]
    struct Payload<'a> {
        log: &'a str,
    }

    let log = log.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        let _resp: String = tauri::invoke("bolt_panic", &Payload { log: &log })
            .await
            .unwrap();
    });
}

pub fn set_html(id: &str, content: String) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, id).unwrap();

    div.set_inner_html(&content);
}

pub fn get_method() -> Method {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "methodselect").unwrap();

    let select = div.dyn_into::<web_sys::HtmlSelectElement>().unwrap();

    let value = select.value();

    let value = match value.as_str() {
        "get" => Method::GET,
        "post" => Method::POST,

        _ => {
            bolt_panic("invalid method");

            Method::GET
        }
    };

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

pub fn get_body() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "reqbody").unwrap();

    let input = div.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();

    let value = input.value();

    return value;
}

pub fn get_header(index: usize) -> Vec<String> {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let key =
        web_sys::Document::get_element_by_id(&doc, &("headerkey".to_string() + &index.to_string()))
            .unwrap();
    let value = web_sys::Document::get_element_by_id(
        &doc,
        &("headervalue".to_string() + &index.to_string()),
    )
    .unwrap();

    let key = key.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let value = value.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let result = vec![key.value(), value.value()];

    return result;
}

pub fn get_param(index: usize) -> Vec<String> {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let key =
        web_sys::Document::get_element_by_id(&doc, &("paramkey".to_string() + &index.to_string()))
            .unwrap();
    let value = web_sys::Document::get_element_by_id(
        &doc,
        &("paramvalue".to_string() + &index.to_string()),
    )
    .unwrap();

    let key = key.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let value = value.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let result = vec![key.value(), value.value()];

    return result;
}

pub fn switch_req_tab(index: u8) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let req_body_tab = web_sys::Document::get_element_by_id(&doc, "req_body_tab").unwrap();
    let req_params_tab = web_sys::Document::get_element_by_id(&doc, "req_params_tab").unwrap();
    let req_headers_tab = web_sys::Document::get_element_by_id(&doc, "req_headers_tab").unwrap();

    match index {
        1 => {
            req_body_tab.class_list().add_1("tabSelected").unwrap();

            req_params_tab.class_list().remove_1("tabSelected").unwrap();

            req_headers_tab
                .class_list()
                .remove_1("tabSelected")
                .unwrap();
        }

        2 => {
            req_body_tab.class_list().remove_1("tabSelected").unwrap();

            req_params_tab.class_list().add_1("tabSelected").unwrap();

            req_headers_tab
                .class_list()
                .remove_1("tabSelected")
                .unwrap();
        }

        3 => {
            req_body_tab.class_list().remove_1("tabSelected").unwrap();

            req_params_tab.class_list().remove_1("tabSelected").unwrap();

            req_headers_tab.class_list().add_1("tabSelected").unwrap();
        }

        _ => {}
    }
}

pub fn switch_resp_tab(index: u8) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let resp_body_tab = web_sys::Document::get_element_by_id(&doc, "resp_body_tab").unwrap();
    let resp_headers_tab = web_sys::Document::get_element_by_id(&doc, "resp_headers_tab").unwrap();

    match index {
        1 => {
            resp_body_tab.class_list().add_1("tabSelected").unwrap();

            resp_headers_tab
                .class_list()
                .remove_1("tabSelected")
                .unwrap();
        }

        2 => {
            resp_body_tab.class_list().remove_1("tabSelected").unwrap();

            resp_headers_tab.class_list().add_1("tabSelected").unwrap();
        }

        _ => {}
    }
}
