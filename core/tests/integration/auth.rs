use crate::test_with_context;

test_with_context!(test_user_authentication, |ctx| {
    let user = ctx.client.get_current_user().await;
    assert_eq!(user.name, "Test User");
});
