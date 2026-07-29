use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::ToSocketAddrs,
    net::TcpStream,
    path::PathBuf,
    time::Duration,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server_address: "10.85.12.2:7777".to_string(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".halca").join("server_config.json")
}

pub fn load_server_config() -> ServerConfig {
    let path = get_config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str::<ServerConfig>(&content) {
                return config;
            }
        }
    }
    ServerConfig::default()
}

pub fn save_server_config(server_address: &str) {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let config = ServerConfig {
        server_address: server_address.to_string(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&config) {
        let _ = fs::write(path, json);
    }
}

pub fn test_server_connection(addr: &str) -> bool {
    let formatted_addr = if !addr.contains(':') {
        format!("{}:7777", addr)
    } else {
        addr.to_string()
    };

    let timeout = Duration::from_secs(3);
    if let Ok(socket_addrs) = formatted_addr.to_socket_addrs() {
        for socket_addr in socket_addrs {
            if TcpStream::connect_timeout(&socket_addr, timeout).is_ok() {
                return true;
            }
        }
    }
    false
}
