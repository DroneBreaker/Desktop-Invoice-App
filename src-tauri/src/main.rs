// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use actix_web::{HttpServer, App};

use tauri::Window;
mod login;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn toolbar(window: Window) {
    let _ =
        window.eval("document.getElementById('toolbar').innerHTML = include_str!('./index.html');");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![toolbar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // login::main();

    // HttpServer::new(|| {
    //     App.new().service()
    // }).bind("127.0.0.1:8080").run().expect("error while connectimg");

    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     println!("Connection established")
    // }
}
