use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

use anyhow::anyhow;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};

pub const EXE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone, Serialize, Deserialize)]
pub struct Server {
    pub tls: bool,
    pub ports: Box<[u16]>,
    pub address: String,
    pub username: String,
    pub password: String,
}

impl Server {
    pub fn new<S: ToString>(tls: bool, ports: Box<[u16]>, address: S, username: S, password: S) -> Self {
        Self {
            tls,
            ports,
            address: address.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Client {
    /// Address to ping
    pub address: String,
    /// Timeout between pings (in seconds)
    pub timeout: u32,
    /// Email address to send email to if ping failed
    pub email: String,
}

impl Client {
    pub fn new<S: ToString>(address: S, email: S, timeout: u32) -> Self {
        Self {
            address: address.to_string(),
            email: email.to_string(),
            timeout,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub server: Option<Server>,
    pub clients: Vec<Client>,
}

impl Config {
    pub fn write(&self, path: &str) -> anyhow::Result<()> {
        fs::write(path, toml::ser::to_string_pretty(self)?)?;
        Ok(())
    }
}

pub struct ConfigHandle {
    pub conf: Config,
    handle: fs::File,
    dir: AppDirs,
}

impl ConfigHandle {
    pub fn open() -> anyhow::Result<Self> {
        let app_dir =
            AppDirs::new(Some(EXE_NAME), false).ok_or(anyhow!("Failed to get app dirs"))?;
        let config_path = app_dir.config_dir.join("config.toml");

        if !app_dir.config_dir.is_dir() {
            fs::create_dir_all(&app_dir.config_dir)?;
        }

        let mut handle = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(config_path)?;

        let mut content = String::new();
        let _ = handle.read_to_string(&mut content);

        let conf: Config = toml::de::from_str(&content).unwrap_or_default();

        Ok(Self {
            conf,
            handle,
            dir: app_dir,
        })
    }
    /// Save the config file
    pub fn write(&mut self) -> anyhow::Result<()> {
        self.handle
            .write(toml::ser::to_string_pretty(&self.conf)?.as_bytes())
            .map(|_| ())?;
        Ok(())
    }
}
