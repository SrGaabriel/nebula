use dotenvy::dotenv;
use reqwest::{Client, Method, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::task;
use nebula_core::web::routing::auth::{AuthResponse, signup::SignupRequest};
use nebula_core::web::routing::dto::{RealmDto, UserDto};
use nebula_core::web::routing::realms::{RealmObject, create::CreateRealmPayload};
use nebula_core::web::routing::users::UserObject;

struct TestContext {
    client: Client,
    base_url: String,
    token: Option<String>,
    user_response: Option<UserDto>,
    realm_response: Option<RealmDto>,
}

impl TestContext {
    fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            token: None,
            user_response: None,
            realm_response: None,
        }
    }

    async fn signup(&mut self, name: &str, email: &str, password: &str) -> Result<UserDto, Box<dyn std::error::Error>> {
        let signup_data = SignupRequest {
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self.client
            .post(&format!("{}/api/signup", self.base_url))
            .header("Content-Type", "application/json")
            .json(&signup_data)
            .send()
            .await?;

        let auth_response: AuthResponse = response.json().await?;
        self.token = Some(auth_response.token.clone());
        self.user_response = Some(auth_response.user.clone());
        Ok(auth_response.user)
    }

    async fn get_current_user(&self) -> UserDto {
        let user_obj: UserObject = self.authorized_fch(Method::GET, "api/users/@me").await;
        user_obj.user
    }

    async fn create_realm(&mut self, name: &str, description: &str) -> RealmDto {
        let realm_data = CreateRealmPayload {
            name: name.to_string(),
            description: Some(description.to_string()),
        };

        let realm_obj: RealmObject = self.authorized_pst(Method::POST, "api/realms", &realm_data).await;
        self.realm_response = Some(realm_obj.realm.clone());
        realm_obj.realm
    }

    async fn get_realm(&self, realm_id: u64) -> RealmObject {
        self.authorized_fch(Method::GET, &format!("api/realms/{}", realm_id)).await
    }

    async fn authorized_pst<T: serde::Serialize, R: DeserializeOwned>(&self, method: Method, endpoint: &str, body: &T) -> R {
        self
            .authorized_snd(method, endpoint, body)
            .await
            .json()
            .await
            .expect(format!("Failed to parse JSON response at {}", endpoint).as_str())
    }

    async fn authorized_fch<T: DeserializeOwned>(&self, method: Method, endpoint: &str) -> T {
        self
            .authorized_qry(method, endpoint)
            .await
            .json()
            .await
            .expect(format!("Failed to parse JSON response at {}", endpoint).as_str())
    }

    async fn authorized_qry(&self, method: Method, endpoint: &str) -> Response {
        self.client
            .request(method, &format!("{}/{}", self.base_url, endpoint))
            .header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()))
            .send()
            .await
            .unwrap()
    }

    async fn authorized_snd<T : Serialize>(&self, method: Method, endpoint: &str, body: &T) -> Response {
        self.client
            .request(method, &format!("{}/{}", self.base_url, endpoint))
            .header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .expect("Failed to send request")
    }
}

fn get_api_base_url() -> String {
    dotenv().ok();
    let host = std::env::var("REST_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("REST_PORT").unwrap_or_else(|_| "3000".to_string());
    format!("http://{}:{}", host, port)
}

#[tokio::test]
async fn test_complete_flow() {
    let _output = std::process::Command::new("py")
        .arg("migrate.py")
        .arg("fresh")
        .output()
        .expect("Failed to run migration");

    task::spawn(async move {
        nebula_core::run_server().await;
    });

    let base_url = get_api_base_url();
    let mut context = TestContext::new(base_url);

    let signup_response = context.signup("John Doe", "john@doe.com", "password").await.unwrap();
    assert!(context.token.is_some());
    assert!(context.user_response.is_some());
    println!("Signed up user: {:?}", signup_response);

    let user = context.get_current_user().await;
    assert_eq!(user.id, signup_response.id);
    println!("Current user: {:?}", user);

    let created_realm = context
        .create_realm("My First Realm", "Beautiful stuff")
        .await;
    println!("Created realm: {:?}", created_realm);

    let fetched_realm = context.get_realm(created_realm.id.0).await;
    assert_eq!(fetched_realm.realm.id, created_realm.id);
    println!("Fetched realm: {:?}", fetched_realm);

    println!("Test complete flow succeeded");
}