use std::sync::Once;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use crate::client::TestClient;

pub mod auth;
pub mod realm;
pub mod event;
pub mod task;

static INIT: Once = Once::new();

fn init_test_env() {
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
        if std::env::var("LOGGING").is_ok_and(|v| v == "true") {
            let subscriber = FmtSubscriber::builder()
                .with_max_level(Level::TRACE)
                .finish();
            let _result = tracing::subscriber::set_global_default(subscriber);
        }
    });
}

pub struct TestContext {
    pub client: TestClient,
}

impl TestContext {
    pub async fn new() -> Self {
        init_test_env();
        let client = TestClient::new().await;
        Self { client }
    }

    pub async fn create_test_realm(&self) -> nebula_server::web::routing::dto::RealmDto {
        self.client.create_test_realm().await
    }

    pub async fn create_realm(&self, name: &str, description: Option<&str>) -> nebula_server::web::routing::dto::RealmDto {
        self.client.create_realm(name, description).await
    }
}

#[macro_export]
macro_rules! test_with_context {
    ($test_name:ident, |$ctx:ident| $body:block) => {
        #[tokio::test]
        async fn $test_name() {
            let $ctx = $crate::integration::TestContext::new().await;
            $body
        }
    };
}

#[macro_export]
macro_rules! test_with_realm {
    ($test_name:ident, |$ctx:ident, $realm:ident| $body:block) => {
        #[tokio::test]
        async fn $test_name() {
            let $ctx = $crate::integration::TestContext::new().await;
            let $realm = $ctx.create_test_realm().await;
            $body
        }
    };
}
