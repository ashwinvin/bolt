// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tauri::Window;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize)]
struct HttpRequest {
    url: String,
    method: Method,
    body: String,
    headers: Vec<Vec<String>>,
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
fn bolt_panic(log: &str) {
    panic!("{}", log);
}

#[tauri::command]
fn send_request(
    window: Window,
    url: String,
    method: Method,
    body: String,
    headers: Vec<Vec<String>>,
) -> String {
    // bolt_log("Sending request");

    let req = HttpRequest {
        url,
        method,
        body,
        headers,
    };

    std::thread::spawn(move || {
        let resp: HttpResponse = http_send(req);

        let resp = serde_json::to_string(&resp).unwrap();

        window.emit("receive_response", resp).unwrap();
    });

    return "done".to_string();
}

fn http_send(req: HttpRequest) -> HttpResponse {
    let mut resp = HttpResponse {
        status: 0,
        body: String::new(),
        headers: Vec::new(),
        time: 0,
        size: 0,
    };

    let client = reqwest::blocking::Client::new();

    match req.method {
        Method::GET => {
            let mut request = client.get(req.url).body(req.body);

            for h in req.headers {
                if h[0] != "" && h[1] != "" {
                    println!("{} : {}", h[0], h[1]);
                    request = request.header(h[0].clone(), h[1].clone());
                }
            }

            let start = get_timestamp();
            let response = request.send().unwrap();
            let end = get_timestamp();

            let headers = extract_headers(response.headers());

            resp.status = response.status().as_u16();
            resp.time = (end - start) as u32;
            resp.body = response.text().unwrap();
            resp.headers = headers;
            resp.size = resp.body.len() as u64;
        }

        Method::POST => {
            let mut request = client.post(req.url).body(req.body);

            for h in req.headers {
                if h[0] != "" && h[1] != "" {
                    println!("{} : {}", h[0], h[1]);
                    request = request.header(h[0].clone(), h[1].clone());
                }
            }

            let start = get_timestamp();
            let response = request.send().unwrap();
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
        .invoke_handler(tauri::generate_handler![send_request, bolt_log, bolt_panic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
}
