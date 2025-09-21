use crate::app::NebulaApp;
use crate::data::calendar::RecurrenceRule;
use crate::data::snowflake::Snowflake;
use crate::schema::realm_events;
use crate::web::routing::dto::RealmEventDto;
use crate::web::routing::dto::RealmEventOccurrenceDto;
use crate::web::routing::dto::RealmEventOccurrenceList;
use crate::web::routing::error::{ok, NebulaResponse};
use axum::extract::{Path, Query, State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OccurrenceQuery {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

pub async fn get_occurrences(
    Path(realm_id): Path<Snowflake>,
    State(app): State<NebulaApp>,
    Query(query): Query<OccurrenceQuery>
) -> NebulaResponse<RealmEventOccurrenceList> {
    let db = &app.state.read().await.db;
    let events = realm_events::Entity::find()
        .filter(realm_events::Column::RealmId.eq(realm_id))
        .filter(realm_events::Column::StartTime.gte(query.start))
        .filter(realm_events::Column::StartTime.lte(query.end))
        .all(db)
        .await
        .expect("Failed to query events");
    let mut occurrence_dtos = vec![];
    let mut event_dtos = vec![];

    let mut i = 0;
    for event in events {
        let event_dto = RealmEventDto::from_model(&event);
        event_dtos.push(event_dto.clone());

        let recurrence_rule = match event.recurrence {
            Some(encoded) => RecurrenceRule::from_u64(encoded as u64).ok(),
            None => None,
        };
        let occurrences = match &recurrence_rule {
            Some(rule) =>
                rule.generate_occurrences(event.start_time, query.start, query.end, None),
            None => {
                if event.start_time >= query.start && event.start_time <= query.end {
                    vec![event.start_time]
                } else {
                    vec![]
                }
            }
        };
        let event_duration = match event.end_time {
            Some(end) => Some(end - event.start_time),
            None => None
        };


        for occurrence in occurrences {
            occurrence_dtos.push(RealmEventOccurrenceDto {
                event_index: i,
                occurrence_start: occurrence,
                occurrence_end: event_duration.map(|d| occurrence + d)
            });
        }
        i += 1;
    }
    ok(RealmEventOccurrenceList {
        events: event_dtos,
        occurrences: occurrence_dtos
    })
}