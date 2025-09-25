use sea_orm::{Database, DatabaseConnection};
use crate::app::AppConfig;
use migration::{Migrator, MigratorTrait};

pub async fn connect(config: &AppConfig) -> DatabaseConnection {
    let db = Database::connect(config.db_url.as_str())
        .await
        .expect("Failed to connect to the database");
    if config.db_fresh {
        Migrator::fresh(&db).await.unwrap();
    }
    db
}