use sea_orm::{Database, DatabaseConnection};
use crate::app::AppConfig;
use migration::{Migrator, MigratorTrait};

pub async fn connect(config: &AppConfig) -> DatabaseConnection {
    let db = Database::connect(config.db_url.as_str())
        .await
        .expect("Failed to connect to the database");
    if config.db_fresh {
        println!("Running fresh database migration...");
        Migrator::up(&db, None)
            .await
            .expect("Failed to run database migrations");
    }
    db
}