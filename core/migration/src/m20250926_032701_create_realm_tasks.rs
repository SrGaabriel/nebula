use sea_orm_migration::prelude::*;
use crate::m20250914_040704_create_users::Users;
use crate::m20250914_195455_create_realms::Realms;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RealmTasks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RealmTasks::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::RealmId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::AuthorId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::Title)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::DueDate)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::StartDate)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::PlannedFor)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::Priority)
                            .small_integer()
                            .null()
                            .to_owned()
                    )
                    .col(
                        ColumnDef::new(RealmTasks::Completed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(RealmTasks::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_realm_tasks_realm_id")
                            .from(RealmTasks::Table, RealmTasks::RealmId)
                            .to(Realms::Table, Realms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_realm_tasks_author_id")
                            .from(RealmTasks::Table, RealmTasks::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_realm_id")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::RealmId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_author_id")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::AuthorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_completed")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::Completed)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_due_date")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::DueDate)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_planned_for")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::PlannedFor)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_updated_at")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_realm_completed")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::RealmId)
                    .col(RealmTasks::Completed)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_tasks_author_completed")
                    .table(RealmTasks::Table)
                    .col(RealmTasks::AuthorId)
                    .col(RealmTasks::Completed)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RealmTasks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RealmTasks {
    Table,
    Id,
    RealmId,
    AuthorId,
    Title,
    Description,
    DueDate,
    Priority,
    StartDate,
    PlannedFor,
    Completed,
    UpdatedAt,
}