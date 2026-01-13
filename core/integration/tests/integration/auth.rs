use crate::test_with_context;

test_with_context!(test_user_authentication, |ctx| {
    let status = ctx.client.get_current_status().await;
    assert_eq!(status.me.name, "Test User");
});
