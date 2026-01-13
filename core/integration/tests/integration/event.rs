use crate::test_with_realm;
use chrono::{DateTime, Utc};
use nebula_server::web::routing::realms::calendar::events::CreateEventRequest;

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