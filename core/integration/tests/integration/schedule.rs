use chrono::{DateTime, Utc, Weekday};
use rrule::{Frequency, NWeekday, RRule};
use nebula_server::web::routing::realms::calendar::events::CreateEventRequest;
use nebula_server::web::routing::realms::calendar::occurrences::OccurrenceQuery;
use nebula_server::web::routing::realms::task::CreateTaskRequest;
use crate::test_with_realm;

test_with_realm!(test_schedule_retrieval, |ctx, realm| {
    let event_payload = CreateEventRequest {
        name: "Gym".to_string(),
        description: Some("Workout time".to_string()),
        location: Some("Dumbfit".to_string()),
        start_time: DateTime::parse_from_rfc3339("2024-06-01T17:00:00Z").unwrap().with_timezone(&Utc),
        end_time: Some(DateTime::parse_from_rfc3339("2024-06-01T18:00:00Z").unwrap().with_timezone(&Utc)),
        recurrence: Some(
            RRule::new(Frequency::Weekly)
                .by_weekday(vec![
                    NWeekday::Every(Weekday::Mon),
                    NWeekday::Every(Weekday::Wed),
                    NWeekday::Every(Weekday::Fri),
                ])
        )
    };
    ctx.client.create_realm_event(realm.id.0, &event_payload).await;

    let task_payload = CreateTaskRequest {
        title: "Finish Integration Tests".to_string(),
        description: Some("Write and verify integration tests for the API".to_string()),
        due_date: Some(DateTime::parse_from_rfc3339("2024-06-15T23:59:59Z").unwrap().with_timezone(&Utc)),
        start_date: None,
        planned_for: Some(DateTime::parse_from_rfc3339("2024-06-10T12:00:00Z").unwrap().with_timezone(&Utc)),
        priority: Some(2),
        completed: false
    };
    ctx.client.create_task(realm.id.0, &task_payload).await;

    let query = OccurrenceQuery {
        start: DateTime::parse_from_rfc3339("2024-06-01T00:00:00Z").unwrap().with_timezone(&Utc),
        end: DateTime::parse_from_rfc3339("2024-07-01T00:00:00Z").unwrap().with_timezone(&Utc)
    };

    let schedule = ctx.client.get_realm_schedule(realm.id.0, &query).await;

    assert_eq!(schedule.events.len(), 1);
    assert_eq!(schedule.events[0].name, "Gym");
    assert_eq!(schedule.tasks.len(), 1);
    assert_eq!(schedule.tasks[0].title, "Finish Integration Tests");
    assert_eq!(schedule.occurrences.len(), 12);
});
