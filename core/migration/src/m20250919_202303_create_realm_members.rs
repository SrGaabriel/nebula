use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RealmMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RealmMembers::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RealmMembers::RealmId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RealmMembers::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RealmMembers::Permissions)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_realm_members_realm_id")
                    .from(RealmMembers::Table, RealmMembers::RealmId)
                    .to(Realms::Table, Realms::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_realm_members_user_id")
                    .from(RealmMembers::Table, RealmMembers::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_members_realm_user")
                    .table(RealmMembers::Table)
                    .col(RealmMembers::RealmId)
                    .col(RealmMembers::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_members_realm_id")
                    .table(RealmMembers::Table)
                    .col(RealmMembers::RealmId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_realm_members_user_id")
                    .table(RealmMembers::Table)
                    .col(RealmMembers::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_members_user_id")
                    .table(RealmMembers::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_members_realm_id")
                    .table(RealmMembers::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_realm_members_realm_user")
                    .table(RealmMembers::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_realm_members_user_id")
                    .table(RealmMembers::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_realm_members_realm_id")
                    .table(RealmMembers::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(RealmMembers::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RealmMembers {
    Table,
    Id,
    RealmId,
    UserId,
    Permissions,
}

#[derive(DeriveIden)]
enum Realms {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}