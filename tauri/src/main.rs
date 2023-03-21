// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_request, bolt_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
        let resp = http_send(&url, mtd);

        window
            .emit("receive_response", Payload { body: resp.into() })
            .unwrap();
    });
}

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

fn http_send(url: &str, method: Method) -> String {
    match method {
        Method::GET => {
            let resp = reqwest::blocking::get(url).unwrap().text().unwrap();

            return resp;
        }

        Method::POST => {
            let client = reqwest::blocking::Client::new();
            let resp = client.post(url).send().unwrap().text().unwrap();

            return resp;
        }

        Method::NONE => {
            panic!("Invalid method");
        }
    }
}
