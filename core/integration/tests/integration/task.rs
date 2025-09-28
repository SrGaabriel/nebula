use nebula_server::web::routing::realms::task::CreateTaskRequest;
use crate::test_with_realm;

test_with_realm!(test_task_creation, |ctx, realm| {
    let payload = CreateTaskRequest {
        title: "Finish Integration Tests".to_string(),
        description: Some("Write and verify integration tests for the API".to_string()),
        due_date: None,
        start_date: None,
        planned_for: None,
        priority: Some(2),
        completed: false
    };

    let task = ctx.client.create_task(realm.id.0, &payload).await;

    assert_eq!(task.title, "Finish Integration Tests");
    assert_eq!(task.description, Some("Write and verify integration tests for the API".to_string()));
    assert_eq!(task.priority, Some(2));
    assert!(!task.completed);
    assert!(task.id.0 > 0);
});
