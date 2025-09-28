use crate::{test_with_context, test_with_realm};

test_with_context!(test_realm_creation, |ctx| {
    let realm = ctx.create_test_realm().await;

    assert_eq!(realm.name, "Test Realm");
    assert_eq!(realm.description, Some("Test realm description".to_string()));
    assert!(realm.id.0 > 0);
});

test_with_realm!(test_realm_retrieval, |ctx, realm| {
    let fetched_realm = ctx.client.get_realm(realm.id.0).await;

    assert_eq!(fetched_realm.realm.id, realm.id);
    assert_eq!(fetched_realm.realm.name, realm.name);
    assert_eq!(fetched_realm.realm.description, realm.description);
});
