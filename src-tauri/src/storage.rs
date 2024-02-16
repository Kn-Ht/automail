use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};

use crate::email;
use aes_siv::aead::Aead;
use aes_siv::{aead::OsRng, AeadCore, Aes256SivAead, KeyInit};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// The data directory for the app
pub static mut APP_DIR: MaybeUninit<String> = MaybeUninit::uninit();

/// The Key used for encrypting/decrypting. Not synced through GIT.
#[doc(hidden)]
pub const AES_KEY: &[u8] = include_str!("key.txt").as_bytes();

// Encryption
lazy_static! {
    /// Encryptor
    static ref CIPHER: Aes256SivAead = Aes256SivAead::new(AES_KEY.into());
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Hashed username
    pub username: String,
    /// Hashed password
    pub password: String,
    /// The nonce used for encrypting/decrypting
    pub nonce: Box<[u8]>,
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<Self, &'static str> {
        let nonce = Aes256SivAead::generate_nonce(&mut OsRng);

        let encrypt = |data: &str| {
            String::from_utf8(
                CIPHER
                    .encrypt(&nonce, data.as_bytes())
                    .map_err(|_| "Encryption error")?,
            )
            .map_err(|_| "UTF8 error")
        };

        let username = encrypt(username)?;
        let password = encrypt(password)?;

        Ok(Self {
            username,
            password,
            nonce: Box::from(nonce.as_slice()),
        })
    }
}

#[tauri::command]
pub fn make_user(username: &str, password: &str) -> Result<User, &'static str> {
    User::new(username, password)
}

#[tauri::command(async)]
pub fn encrypt_credentials(
    username: &str,
    password: &str,
) -> Result<(String, String), &'static str> {
    Ok(User::new(username, password).and_then(|u| Ok((u.username, u.password)))?)
}
