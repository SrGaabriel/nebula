use sea_orm_migration::{prelude::*, schema::*};
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
                    .table(RealmEvents::Table)
                    .if_not_exists()
                    .col(big_integer(RealmEvents::Id).primary_key())
                    .col(string_len(RealmEvents::Name, 255).not_null())
                    .col(text_null(RealmEvents::Description))
                    .col(string_null(RealmEvents::Location))
                    .col(big_integer(RealmEvents::CreatedBy).not_null())
                    .col(big_integer(RealmEvents::RealmId).not_null())
                    .col(timestamp_with_time_zone(RealmEvents::StartTime).not_null())
                    .col(timestamp_with_time_zone_null(RealmEvents::EndTime))
                    .col(big_integer_null(RealmEvents::Recurrence))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_realm_events_created_by")
                            .from(RealmEvents::Table, RealmEvents::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_realm_events_realm_id")
                            .from(RealmEvents::Table, RealmEvents::RealmId)
                            .to(Realms::Table, Realms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_events_realm_id")
                    .table(RealmEvents::Table)
                    .col(RealmEvents::RealmId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_events_start_time")
                    .table(RealmEvents::Table)
                    .col(RealmEvents::StartTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_events_realm_start_time")
                    .table(RealmEvents::Table)
                    .col(RealmEvents::RealmId)
                    .col(RealmEvents::StartTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_events_created_by")
                    .table(RealmEvents::Table)
                    .col(RealmEvents::CreatedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_events_recurrence")
                    .table(RealmEvents::Table)
                    .col(RealmEvents::Recurrence)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_events_recurrence")
                    .table(RealmEvents::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_events_created_by")
                    .table(RealmEvents::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_events_realm_start_time")
                    .table(RealmEvents::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_events_start_time")
                    .table(RealmEvents::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_events_realm_id")
                    .table(RealmEvents::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(RealmEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RealmEvents {
    Table,
    Id,
    Name,
    Description,
    Location,
    CreatedBy,
    RealmId,
    StartTime,
    EndTime,
    Recurrence,
}
