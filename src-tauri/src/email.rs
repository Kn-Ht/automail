use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SmtpLogin {
    pub username: String,
    pub password: String,
}

impl SmtpLogin {
    pub fn new<S: ToString>(username: S, password: S) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string()
        }
    }
}

pub struct Email {
    smtp_login: Option<SmtpLogin>
}

impl Email {
    pub fn new() -> Self {
        Self {
            smtp_login: None
        }
    }
}

lazy_static! {
    static ref EMAIL: Email = Email::new();
}

#[tauri::command(async)]
pub fn smtp_login() -> &'static Option<SmtpLogin> {
    &EMAIL.smtp_login
}

#[tauri::command(async)]
pub fn send_email(offline_addr: &str, timeout: u64) {
    todo!()
}

