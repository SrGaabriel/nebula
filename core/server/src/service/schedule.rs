use std::str::FromStr;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use rrule::{RRule, Tz};
use sea_orm::{DbErr, EntityTrait};
use sea_orm::Condition;
use crate::data::snowflake::Snowflake;
use crate::schema::realm_events;
use crate::web::routing::dto::{RealmEventDto, RealmEventOccurrenceDto, RealmScheduleDto};

pub async fn get_realm_schedule(
    db: &sea_orm::DatabaseConnection,
    realm_id: Snowflake,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>
) -> Result<RealmScheduleDto, DbErr> {
    let events = realm_events::Entity::find()
        .filter(realm_events::Column::RealmId.eq(realm_id))
        .filter(realm_events::Column::StartTime.gte(start))
        .filter(realm_events::Column::StartTime.lte(end))
        .all(db)
        .await?;
    let mut occurrence_dtos = vec![];
    let mut event_dtos = vec![];

    let tasks = crate::schema::realm_tasks::Entity::find()
        .filter(crate::schema::realm_tasks::Column::RealmId.eq(realm_id))
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(crate::schema::realm_tasks::Column::PlannedFor.gte(start))
                        .add(crate::schema::realm_tasks::Column::PlannedFor.lte(end))
                )
                .add(
                    Condition::all()
                        .add(crate::schema::realm_tasks::Column::DueDate.gte(start))
                        .add(crate::schema::realm_tasks::Column::DueDate.lte(end))
                )
                .add(
                    Condition::all()
                        .add(crate::schema::realm_tasks::Column::StartDate.gte(start))
                        .add(crate::schema::realm_tasks::Column::StartDate.lte(end))
                )
        )
        .all(db)
        .await?;
    let task_dtos = tasks
        .into_iter()
        .map(|t| crate::web::routing::dto::TaskDto::from_model(t))
        .collect();

    let mut i = 0;
    for event in events {
        let event_dto = RealmEventDto::from_model(&event);
        event_dtos.push(event_dto.clone());

        let recurrence_rule = match event.recurrence {
            Some(encoded) => RRule::from_str(&encoded).ok(),
            None => None,
        };
        let occurrences = match recurrence_rule {
            Some(rule) => {
                let start_utc = event.start_time.with_timezone(&Tz::UTC);
                let end_utc = end.with_timezone(&Tz::UTC);
                rule
                    .build(start_utc)
                    .expect("Error building recurrence rule")
                    .before(end_utc)
                    .after(start_utc)
                    .all(1000)
                    .dates
                    .into_iter()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .collect()
            },
            None => {
                if event.start_time >= start && event.start_time <= end {
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
    Ok(RealmScheduleDto {
        events: event_dtos,
        tasks: task_dtos,
        occurrences: occurrence_dtos,
    })
}