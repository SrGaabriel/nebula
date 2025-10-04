use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_orm::RelationTrait;
use axum::Extension;
use axum::extract::State;
use sea_orm::EntityTrait;
use sea_orm::JoinType;
use sea_orm::QuerySelect;
use crate::app::NebulaApp;
use crate::schema::{realm_members, realms, users};
use crate::web::routing::dto::{SelfStatusDto, RealmDto, UserDto};
use crate::web::routing::error::{NebulaResponse, ok};

pub async fn get_self_status(
    Extension(user): Extension<users::Model>,
    State(app): State<NebulaApp>
) -> NebulaResponse<SelfStatusDto> {
    let realms = realms::Entity::find()
        .join(JoinType::InnerJoin, realms::Relation::RealmMembers.def())
        .filter(realm_members::Column::UserId.eq(user.id))
        .all(&app.db)
        .await
        .expect("Failed to query realms");

    let realms_dto: Vec<RealmDto> = realms
        .iter()
        .map(|realm| RealmDto::from_model(realm))
        .collect();

    let status = SelfStatusDto {
        realms: realms_dto,
        me: UserDto::from_model(&user),
    };

    ok(status)
}