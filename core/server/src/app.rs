use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use async_nats::Client;
use hmac::{Hmac, Mac};
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct NebulaApp {
    pub config: AppConfig,
    pub state: SharedState,
    pub cableway: Client,
    pub db: DatabaseConnection
}

#[derive(Debug)]
pub struct AppState {
}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub rest_addr: SocketAddr,
    pub cableway_addr: SocketAddr,
    pub db_url: String,
    pub db_fresh: bool,
    pub argon_salt: SaltString,
    pub jwt_key: Hmac<Sha256>,
    pub argon2: Argon2<'static>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let rest_host: IpAddr = get_required_env("REST_HOST");
        let rest_port: u16 = get_required_env("REST_PORT");

        let cableway_host: IpAddr = get_required_env("CABLEWAY_HOST");
        let cableway_port: u16 = get_required_env("CABLEWAY_PORT");

        let db_url: String = get_required_env("DATABASE_URL");

        let argon_salt_str: String = get_required_env("ARGON_SALT");
        let argon_salt = SaltString::from_b64(&argon_salt_str)
            .expect("Failed to create Argon2 salt from environment variable");
        
        let jwt_secret: String = get_required_env("JWT_SECRET");
        let jwt_key = Hmac::<Sha256>::new_from_slice(jwt_secret.as_bytes())
            .expect("Failed to create JWT key from environment variable");

        let db_fresh = std::env::var("DATABASE_REFRESH")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase() == "true";

        AppConfig {
            rest_addr: SocketAddr::new(rest_host, rest_port),
            cableway_addr: SocketAddr::new(cableway_host, cableway_port),
            db_url,
            db_fresh,
            argon_salt,
            jwt_key,
            argon2: Argon2::default(),
        }
    }
}

fn get_required_env<T : FromStr>(var: &str) -> T where <T as FromStr>::Err: Debug {
    std::env::var(var)
        .expect(&format!("Missing required environment variable: {}", var))
        .parse()
        .expect(&format!("Environment variable formated incorrectly: {}", var))
}