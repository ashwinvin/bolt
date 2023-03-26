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
    time: u32,
    size: u64,
}

#[derive(Serialize)]
struct AppState {
    count: i32,
    response: HttpResponse,
}

impl AppState {
    fn new() -> Self {
        Self {
            count: 0,
            response: HttpResponse {
                status: 0,
                body: String::new(),
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

    let mut state = GLOBAL_STATE.lock().unwrap();
    state.count += 1;
    println!("count is {}", state.count);

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
        time: 0,
        size: 0,
    };

    match method {
        Method::GET => {
            let start = get_timestamp();
            let response = reqwest::blocking::get(url).unwrap();
            let end = get_timestamp();

            resp.status = response.status().as_u16();
            resp.time = (end - start) as u32;
            resp.body = response.text().unwrap();
            resp.size = resp.body.len() as u64;

            return resp;
        }

        Method::POST => {
            let client = reqwest::blocking::Client::new();

            let start = get_timestamp();
            let response = client.post(url).send().unwrap();
            let end = get_timestamp();

            resp.status = response.status().as_u16();
            resp.time = (end - start) as u32;
            resp.body = response.text().unwrap();
            resp.size = resp.body.len() as u64;

            return resp;
        }

        Method::NONE => {
            panic!("Invalid method");
        }
    }
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
