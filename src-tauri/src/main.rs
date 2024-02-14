// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod email;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            email::smtp_login,
            email::send_email
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| eprintln!("[automail] error: {e}"));
}
