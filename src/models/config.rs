use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server
}

#[derive(Deserialize)]
pub struct Server {
    pub ip: std::net::Ipv4Addr,
    pub port: u16
}
