use sea_orm_migration::prelude::*;
use super::m20250914_040704_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Realms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Realms::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Realms::Name).string().not_null())
                    .col(ColumnDef::new(Realms::Description).string())
                    .col(ColumnDef::new(Realms::OwnerId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-realms-owner")
                            .from(Realms::Table, Realms::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Realms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Realms {
    Table,
    Id,
    Name,
    Description,
    OwnerId,
}