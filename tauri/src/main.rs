// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Window;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    NONE,
}

#[derive(Clone, serde::Serialize)]
struct HttpResponse {
    status: u16,
    body: String,
    time: u32,
    size: u64,
}

#[tauri::command]
fn bolt_log(log: &str) {
    println!("{}", log);
}

#[tauri::command]
fn send_request(window: Window, url: &str, method: &str) {
    let mtd = match method {
        "post" => Method::POST,
        "get" => Method::GET,
        &_ => Method::NONE,
    };

    let url = url.to_string();
    let _handle = std::thread::spawn(move || {
        let resp: HttpResponse = http_send(&url, mtd);

        window
            .emit(
                "receive_response",
                HttpResponse {
                    body: resp.body,
                    size: resp.size,
                    status: resp.status,
                    time: resp.time,
                },
            )
            .unwrap();
    });
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
