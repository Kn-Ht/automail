pub struct PingClient {
    /// Address to ping
    addr: String,
    /// Timeout between pings (in seconds)
    timeout: u32,
}

pub struct Config {
    clients: Vec<PingClient>
}

impl Config {
    pub fn read(contents: &str) -> Self {
        toml::
    }
}