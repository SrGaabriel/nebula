pub use sea_orm_migration::prelude::*;

pub mod m20250914_040704_create_users;
pub mod m20250914_195455_create_realms;
pub mod m20250919_202303_create_realm_members;
pub mod m20250921_015955_create_realm_events;
pub mod m20250926_032701_create_realm_tasks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250914_040704_create_users::Migration),
             Box::new(m20250914_195455_create_realms::Migration),
             Box::new(m20250919_202303_create_realm_members::Migration),
             Box::new(m20250921_015955_create_realm_events::Migration),
             Box::new(m20250926_032701_create_realm_tasks::Migration)
        ]
    }
}
