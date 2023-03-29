// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tauri::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    NONE,
}

#[derive(Clone, Serialize)]
struct HttpResponse {
    status: u16,
    body: String,
    headers: Vec<Vec<String>>,
    time: u32,
    size: u64,
}

#[derive(Serialize)]
struct AppState {
    response: HttpResponse,
}

impl AppState {
    fn new() -> Self {
        Self {
            response: HttpResponse {
                status: 0,
                body: String::new(),
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

#[tauri::command]
fn bolt_log(log: &str) -> String {
    println!("{}", log);

    return "done".to_string();
}

#[tauri::command]
fn send_request(window: Window, url: &str, method: &str) -> String {
    bolt_log("Sending request");

    let mtd = match method {
        "post" => Method::POST,
        "get" => Method::GET,
        &_ => Method::NONE,
    };

    let url = url.to_string();
    std::thread::spawn(move || {
        let resp: HttpResponse = http_send(&url, mtd);

        let resp = serde_json::to_string(&resp).unwrap();

        window.emit("receive_response", resp).unwrap();
    });

    return "done".to_string();
}

fn http_send(url: &str, method: Method) -> HttpResponse {
    let mut resp = HttpResponse {
        status: 0,
        body: String::new(),
        headers: Vec::new(),
        time: 0,
        size: 0,
    };

    match method {
        Method::GET => {
            let start = get_timestamp();
            let response = reqwest::blocking::get(url).unwrap();
            let end = get_timestamp();

            let headers = extract_headers(response.headers());

            resp.status = response.status().as_u16();
            resp.time = (end - start) as u32;
            resp.body = response.text().unwrap();
            resp.headers = headers;
            resp.size = resp.body.len() as u64;
        }

        Method::POST => {
            let client = reqwest::blocking::Client::new();

            let start = get_timestamp();
            let response = client.post(url).send().unwrap();
            let end = get_timestamp();

            let headers = extract_headers(response.headers());

            resp.status = response.status().as_u16();
            resp.time = (end - start) as u32;
            resp.body = response.text().unwrap();
            resp.headers = headers;
            resp.size = resp.body.len() as u64;
        }

        Method::NONE => {
            panic!("Invalid method");
        }
    }

    let mut state = GLOBAL_STATE.lock().unwrap();
    state.response = resp.clone();

    return resp;
}

fn extract_headers(map: &reqwest::header::HeaderMap) -> Vec<Vec<String>> {
    let mut headers: Vec<Vec<String>> = Vec::new();

    for (key, value) in map.iter() {
        let mut header: Vec<String> = Vec::new();

        header.push(key.to_string());
        header.push(value.to_str().unwrap().to_string());

        headers.push(header);
    }

    return headers;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_request, bolt_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
}
