use crate::app::AppConfig;
use crate::schema::users;
use jwt::VerifyWithKey;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::collections::BTreeMap;

pub async fn authenticate(
    config: &AppConfig,
    db: &DatabaseConnection,
    token: String
) -> Result<users::Model, ()> {
    let claims: Result<BTreeMap<String, String>, jwt::error::Error> = token
        .verify_with_key(&config.jwt_key);
    if claims.is_err() {
        return Err(());
    }
    let claims = claims.unwrap();

    let user_id_str: Option<&String> = claims.get("user_id");
    if user_id_str.is_none() {
        return Err(());
    }
    let user_id = user_id_str.unwrap().parse::<u64>();
    if user_id.is_err() {
        return Err(());
    }
    let user_query = users::Entity::find_by_id(user_id.unwrap())
        .one(db)
        .await;
    
    if user_query.is_err() {
        return Err(());
    }
    let user = user_query.unwrap();
    if user.is_none() {
        return Err(());
    }
    Ok(user.unwrap())
}