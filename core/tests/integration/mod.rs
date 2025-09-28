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
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        let _result = tracing::subscriber::set_global_default(subscriber);

        if std::env::var("TEST_RUN_SERVER").is_ok_and(|v| v == "true") {
            println!("Starting server for tests...");
            tokio::task::spawn(async move {
                nebula_core::run_server().await;
            });
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
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

    pub async fn create_test_realm(&self) -> nebula_core::web::routing::dto::RealmDto {
        self.client.create_test_realm().await
    }

    pub async fn create_realm(&self, name: &str, description: Option<&str>) -> nebula_core::web::routing::dto::RealmDto {
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
