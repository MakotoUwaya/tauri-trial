// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command(invoke_message: &str) -> String {
  println!("I was invoked from JS, with this message: {}", invoke_message);
  String::from("success")
}

#[tauri::command]
fn simple_command() {
    println!("I was invoked from JS!");
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let _id = app.listen_global("front-to-back", |event| {
                println!(
                    "got front-to-back with payload {:?}",
                    event.payload().unwrap()
                )
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            simple_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
