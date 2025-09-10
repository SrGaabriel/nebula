use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use async_nats::Client;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct NebulaApp {
    pub config: AppConfig,
    pub state: SharedState,
    pub cableway: Client
}

#[derive(Debug)]
pub struct AppState {
    pub jwt_key: Hmac<Sha256>
}

impl AppState {
    pub fn from_env() -> Self {
        let jwt_key: String = get_required_env("JWT_KEY");

        let key = Hmac::<Sha256>::new_from_slice(jwt_key.as_bytes())
            .expect("Failed to create HMAC");

        AppState {
            jwt_key: key
        }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone, Copy, Debug)]
pub struct AppConfig {
    pub rest_address: SocketAddr,
    pub cableway_address: SocketAddr
}

impl AppConfig {
    pub fn from_env() -> Self {
        let rest_host: IpAddr = get_required_env("REST_HOST");
        let rest_port: u16 = get_required_env("REST_PORT");

        let cableway_host: IpAddr = get_required_env("CABLEWAY_HOST");
        let cableway_port: u16 = get_required_env("CABLEWAY_PORT");

        AppConfig {
            rest_address: SocketAddr::new(rest_host, rest_port),
            cableway_address: SocketAddr::new(cableway_host, cableway_port)
        }
    }
}

fn get_required_env<T : FromStr>(var: &str) -> T where <T as FromStr>::Err: Debug {
    std::env::var(var)
        .expect(&format!("Missing required environment variable: {}", var))
        .parse()
        .expect(&format!("Environment variable formated incorrectly: {}", var))
}