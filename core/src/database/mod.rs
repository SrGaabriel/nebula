use crate::app::AppConfig;

pub async fn connect(config: &AppConfig) -> sea_orm::DatabaseConnection {
    let database_url = format!("postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name
    );
    sea_orm::Database::connect(database_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn run_migrations() {

}