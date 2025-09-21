use crate::web::routing::dto::RealmEventDto;

pub mod events;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct RealmEventObject {
    pub event: RealmEventDto
}