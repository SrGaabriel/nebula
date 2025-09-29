use nebula_server::web::routing::auth::signup::SignupRequest;
use nebula_server::web::routing::auth::AuthResponse;
use nebula_server::web::routing::dto::{RealmDto, RealmEventDto, TaskDto, UserDto};
use nebula_server::web::routing::realms::calendar::events::CreateEventRequest;
use nebula_server::web::routing::realms::calendar::RealmEventObject;
use nebula_server::web::routing::realms::create::CreateRealmPayload;
use nebula_server::web::routing::realms::task::{CreateTaskRequest, TaskObject};
use nebula_server::web::routing::realms::RealmObject;
use nebula_server::web::routing::users::UserObject;
use reqwest::{Client, Method, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct TestClient {
    client: ClientWithMiddleware,
    base_url: String,
    pub token: String,
}

impl TestClient {
    pub fn with_token(token: &str) -> Self {
        let port = std::env::var("REST_PORT")
            .expect("REST_PORT environment variable not set");
        let base_url = format!("http://localhost:{port}");
        let client = ClientBuilder::new(Client::new()).build();
        Self { client, base_url, token: token.to_owned() }
    }

    pub async fn login() -> Self {
        let port = std::env::var("REST_PORT")
            .expect("REST_PORT environment variable not set");
        let base_url = format!("http://localhost:{port}");
        let client = ClientBuilder::new(Client::new()).build();
        let token = login(&client, &base_url).await;

        Self::with_token(&token)
    }

    pub async fn create_test_realm(&self) -> RealmDto {
        let payload = CreateRealmPayload {
            name: "Test Realm".to_string(),
            description: Some("Test realm description".to_string()),
        };

        let realm_obj: RealmObject = self.post("api/realms", &payload).await;
        realm_obj.realm
    }

    pub async fn create_realm(&self, name: &str, description: Option<&str>) -> RealmDto {
        let payload = CreateRealmPayload {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        };

        let realm_obj: RealmObject = self.post("api/realms", &payload).await;
        realm_obj.realm
    }

    pub async fn get_current_user(&self) -> UserDto {
        let user_obj: UserObject = self.get("api/users/@me").await;
        user_obj.user
    }

    pub async fn get_realm(&self, realm_id: u64) -> RealmObject {
        self.get(&format!("api/realms/{}", realm_id)).await
    }

    pub async fn create_realm_event(&self, realm_id: u64, payload: &CreateEventRequest) -> RealmEventDto {
        let event_obj: RealmEventObject = self
            .post(&format!("api/realms/{}/calendar/events", realm_id), payload)
            .await;
        event_obj.event
    }

    pub async fn get_realm_schedule<P: Serialize>(&self, realm_id: u64, query: &P) -> nebula_server::web::routing::dto::RealmEventOccurrenceList {
        self.get_with_query(&format!("api/realms/{}/calendar/schedule", realm_id), query).await
    }

    pub async fn create_task(&self, realm_id: u64, payload: &CreateTaskRequest) -> TaskDto {
        let task_obj: TaskObject = self
            .post(&format!("api/realms/{}/tasks", realm_id), payload)
            .await;
        task_obj.task
    }

    async fn post<T: Serialize, R: DeserializeOwned>(&self, endpoint: &str, body: &T) -> R {
        let response = self
            .request(Method::POST, endpoint)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .expect("Failed to send request");

        parse_response(endpoint, response).await
    }

    async fn get<R: DeserializeOwned>(&self, endpoint: &str) -> R {
        let response = self
            .request(Method::GET, endpoint)
            .send()
            .await
            .unwrap();

        parse_response(endpoint, response).await
    }

    async fn get_with_query<P: Serialize, R: DeserializeOwned>(&self, endpoint: &str, params: &P) -> R {
        let response = self
            .request(Method::GET, endpoint)
            .query(params)
            .send()
            .await
            .unwrap();

        parse_response(endpoint, response).await
    }

    fn request(&self, method: Method, endpoint: &str) -> RequestBuilder {
        self.client
            .request(method, &format!("{}/{}", self.base_url, endpoint))
            .header("Authorization", format!("Bearer {}", self.token))
    }
}

async fn login(client: &ClientWithMiddleware, base_url: &str) -> String {
    let signup_data = SignupRequest {
        name: "Test User".to_string(),
        email: format!("test{}@example.com", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)),
        password: "password".to_string(),
    };

    let response = client
        .post(&format!("{}/api/signup", base_url))
        .header("Content-Type", "application/json")
        .json(&signup_data)
        .send()
        .await
        .expect("Failed to signup");

    let auth_response: AuthResponse = response.json().await.expect("Failed to parse auth response");
    auth_response.token
}

async fn parse_response<R: DeserializeOwned>(endpoint: &str, response: Response) -> R {
    if response.status().is_success() {
        let response_body = response.text().await.unwrap();
        serde_json::from_str(&response_body)
            .unwrap_or_else(|err| panic!("Failed to deserialize response at {}: {} for body {}", endpoint, err, response_body))
    } else {
        panic!("Request to {} failed with status {}, body: {}", endpoint, response.status(), response.text().await.unwrap());
    }
}