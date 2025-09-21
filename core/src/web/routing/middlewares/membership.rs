use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::Response;
use sea_orm::EntityTrait;
use crate::app::NebulaApp;
use crate::data::snowflake::Snowflake;
use crate::data::permissions::{BitwisePermissions, RealmPermissions};
use crate::schema::realms;

pub async fn authorize_membership_with_permissions(
    Path(realm_id): Path<Snowflake>,
    State(app): State<NebulaApp>,
    mut req: Request,
    next: Next,
    required_perms: Option<RealmPermissions>,
) -> Response {
    let user = req.extensions().get::<crate::schema::users::Model>();
    if user.is_none() {
        panic!("User not found in request extensions");
    }
    let user = user.unwrap();
    let realm = realms::Entity::find_by_id(realm_id)
        .one(&app.state.read().await.db)
        .await
        .expect("Failed to query realm");

    if realm.is_none() {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(Body::empty())
            .expect("Realm was not found")
    }

    let realm = realm.unwrap();
    let membership = crate::schema::realm_members::Entity::find_membership(
        &app.state.read().await.db,
        realm.id,
        user.id
    ).await.expect("Failed to query realm membership");

    if membership.is_none() {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::FORBIDDEN)
            .body(Body::empty())
            .expect("Membership was not found")
    }

    let membership = membership.unwrap();
    let user_permissions = RealmPermissions::new(membership.permissions);

    if let Some(required) = required_perms && !user_permissions.contains_all(&required) {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::FORBIDDEN)
            .body(Body::empty())
            .expect("Insufficient permissions")
    }

    req.extensions_mut().insert(membership);
    req.extensions_mut().insert(realm);

    next.run(req).await
}

#[macro_export]
macro_rules! realm_membership {
    ($state:expr) => {
        axum::middleware::from_fn_with_state($state.clone(), |path, state, req, next| async move {
            crate::web::routing::middlewares::membership::authorize_membership_with_permissions(
                path,
                state,
                req,
                next,
                None
            ).await
        })
    };
    ($state:expr, [$($perm:ident),*]) => {
        axum::middleware::from_fn_with_state($state.clone(), |path, state, req, next| async move {
            crate::web::routing::middlewares::membership::authorize_membership_with_permissions(
                path,
                state,
                req,
                next,
                Some(crate::data::permissions::RealmPermissions::from_slice(&[
                    $(crate::data::permissions::RealmPermission::$perm,)*
                ]))
            ).await
        })
    };
}