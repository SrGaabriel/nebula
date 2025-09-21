use crate::web::routing::dto::RealmEventDto;

pub mod events;
pub mod occurrences;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct RealmEventObject {
    pub event: RealmEventDto
}