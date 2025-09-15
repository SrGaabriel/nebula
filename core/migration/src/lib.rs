pub use sea_orm_migration::prelude::*;

mod m20250914_040704_create_users;
mod m20250914_195455_create_realms;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250914_040704_create_users::Migration),
            Box::new(m20250914_195455_create_realms::Migration),
        ]
    }
}
