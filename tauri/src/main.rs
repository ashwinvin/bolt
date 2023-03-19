// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_btn, bolt_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn bolt_log(log: &str) {
    println!("{}", log);
}

#[tauri::command]
async fn send_btn(window: Window) {
    println!("tauri received from yew");

    call_yew(window);
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn call_yew(window: Window) {
    bolt_log("tauri is sending to yew");

    window
        .emit(
            "yew_func",
            Payload {
                message: "Hello Yew!!".into(),
            },
        )
        .unwrap();
}
