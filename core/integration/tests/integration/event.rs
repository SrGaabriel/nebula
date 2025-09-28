use chrono::{DateTime, Utc, Weekday};
use rrule::{Frequency, NWeekday, RRule};
use nebula_server::web::routing::realms::calendar::events::CreateEventRequest;
use nebula_server::web::routing::realms::calendar::occurrences::OccurrenceQuery;
use crate::test_with_realm;

test_with_realm!(test_event_creation, |ctx, realm| {
    let payload = CreateEventRequest {
        name: "Gym".to_string(),
        description: Some("Workout time".to_string()),
        location: Some("Dumbfit".to_string()),
        start_time: DateTime::parse_from_rfc3339("2024-06-01T17:00:00Z").unwrap().with_timezone(&Utc),
        end_time: Some(DateTime::parse_from_rfc3339("2024-06-01T18:00:00Z").unwrap().with_timezone(&Utc)),
        recurrence: None
    };

    let event = ctx.client.create_realm_event(realm.id.0, &payload).await;

    assert_eq!(event.name, "Gym");
    assert_eq!(event.description, Some("Workout time".to_string()));
    assert_eq!(event.location, Some("Dumbfit".to_string()));
    assert!(event.id.0 > 0);
});

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

    let query = OccurrenceQuery {
        start: DateTime::parse_from_rfc3339("2024-06-01T00:00:00Z").unwrap().with_timezone(&Utc),
        end: DateTime::parse_from_rfc3339("2024-07-01T00:00:00Z").unwrap().with_timezone(&Utc)
    };

    let schedule = ctx.client.get_realm_schedule(realm.id.0, &query).await;

    assert!(!schedule.events.is_empty());
    assert!(!schedule.occurrences.is_empty());
    assert_eq!(schedule.occurrences.len(), 12);
});
