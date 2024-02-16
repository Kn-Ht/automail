use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::storage::User;

#[derive(Clone)]
pub struct Email {
    smtp_login: Option<User>
}

impl Email {
    pub fn new() -> Self {
        Self {
            smtp_login: None
        }
    }
}

lazy_static! {
    static ref EMAIL: Mutex<Email> = Mutex::new(Email::new());
}

#[tauri::command(async)]
pub fn smtp_login() -> Option<User> {
    EMAIL.lock().unwrap().smtp_login.clone()
}

#[tauri::command(async)]
pub fn smtp_set_login(to: User) {
    let mut guard = EMAIL.lock().unwrap();
    guard.smtp_login = Some(to);
}
 
#[tauri::command(async)]
pub fn send_email(offline_addr: &str, timeout: u64) {
    todo!()
}

