use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use argon2::password_hash::SaltString;
use argon2::Argon2;
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
    pub db: sea_orm::DatabaseConnection,
    pub jwt_key: Hmac<Sha256>,
    pub argon: Argon2<'static>
}

impl AppState {
    pub fn new_filling_env_defaults(db: sea_orm::DatabaseConnection) -> Self {
        AppState {
            db,
            jwt_key: Self::from_env_jwt_key(),
            argon: Argon2::default()
        }
    }

    pub fn from_env_jwt_key() -> Hmac<Sha256> {
        let jwt_key: String = get_required_env("JWT_KEY");

        Hmac::<Sha256>::new_from_slice(jwt_key.as_bytes())
            .expect("Failed to create HMAC")
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub rest_addr: SocketAddr,
    pub cableway_addr: SocketAddr,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub argon_salt: SaltString
}

impl AppConfig {
    pub fn from_env() -> Self {
        let rest_host: IpAddr = get_required_env("REST_HOST");
        let rest_port: u16 = get_required_env("REST_PORT");

        let cableway_host: IpAddr = get_required_env("CABLEWAY_HOST");
        let cableway_port: u16 = get_required_env("CABLEWAY_PORT");

        let db_name: String = get_required_env("DB_NAME");
        let db_user: String = get_required_env("DB_USER");
        let db_password: String = get_required_env("DB_PASSWORD");
        let db_host: String = get_required_env("DB_HOST");
        let db_port: u16 = get_required_env("DB_PORT");

        let argon_salt_str: String = get_required_env("ARGON_SALT");
        let argon_salt = SaltString::from_b64(&argon_salt_str)
            .expect("Failed to create Argon2 salt from environment variable");

        AppConfig {
            rest_addr: SocketAddr::new(rest_host, rest_port),
            cableway_addr: SocketAddr::new(cableway_host, cableway_port),
            db_name,
            db_user,
            db_password,
            db_host,
            db_port,
            argon_salt
        }
    }
}

fn get_required_env<T : FromStr>(var: &str) -> T where <T as FromStr>::Err: Debug {
    std::env::var(var)
        .expect(&format!("Missing required environment variable: {}", var))
        .parse()
        .expect(&format!("Environment variable formated incorrectly: {}", var))
}