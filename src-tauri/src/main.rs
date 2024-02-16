// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, WindowBuilder, WindowUrl};

mod email;
mod storage;

#[tauri::command(async)]
fn open_settings_window(app: AppHandle) -> Result<(), tauri::Error> {
    WindowBuilder::new(&app, "Settings", WindowUrl::App("settings.html".into()))
        .fullscreen(false)
        .resizable(false)
        .title("Settings")
        .center()
        .enable_clipboard_access()
        .inner_size(400.0, 300.0)
        .focused(true)
        .build()
        .map(|_| ())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            email::smtp_login,
            email::send_email,
            open_settings_window,
            storage::encrypt_credentials,
            storage::make_user
        ])
        .setup(|app| {
            let dir = app.handle().path_resolver().app_local_data_dir().unwrap();
            unsafe {
                storage::APP_DIR.write(dir.to_str().unwrap().to_owned());
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
