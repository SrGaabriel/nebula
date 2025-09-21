use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use reqwest::{Client, Method, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::task;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use nebula_core::data::calendar::RecurrenceRule;
use nebula_core::web::routing::auth::{AuthResponse, signup::SignupRequest};
use nebula_core::web::routing::dto::{RealmDto, RealmEventDto, RealmEventOccurrenceList, UserDto};
use nebula_core::web::routing::realms::{RealmObject, create::CreateRealmPayload};
use nebula_core::web::routing::realms::calendar::events::CreateEventRequest;
use nebula_core::web::routing::realms::calendar::occurrences::OccurrenceQuery;
use nebula_core::web::routing::realms::calendar::RealmEventObject;
use nebula_core::web::routing::users::UserObject;

#[derive(Default)]
struct TestContext {
    client: ClientWithMiddleware,
    base_url: String,
    token: Option<String>,
    user_response: Option<UserDto>,
    realm_response: Option<RealmDto>,
    realm_event_response: Option<RealmEventDto>,
}

impl TestContext {
    fn new(base_url: String) -> Self {
        Self {
            client: ClientBuilder::new(Client::new())
                .build(),
            base_url,
            ..Default::default()
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

    async fn create_realm_event(&mut self) -> RealmEventDto {
        let realm_id = self.realm_response.as_ref().unwrap().id;
        let event = CreateEventRequest {
            name: "Gym".to_string(),
            description: Some("Workout time".to_string()),
            location: Some("Dumbfit".to_string()),
            start_time: DateTime::parse_from_rfc3339("2024-06-01T17:00:00Z").unwrap().with_timezone(&Utc),
            end_time: Some(DateTime::parse_from_rfc3339("2024-06-01T18:00:00Z").unwrap().with_timezone(&Utc)),
            recurrence: Some(RecurrenceRule::weekly(1, vec![chrono::Weekday::Mon, chrono::Weekday::Wed, chrono::Weekday::Fri]))
        };
        let event = self
            .authorized_pst::<CreateEventRequest, RealmEventObject>(Method::POST, &format!("api/realms/{realm_id}/calendar/events"), &event)
            .await
            .event;
        self.realm_event_response = Some(event.clone());
        event
    }

    async fn get_realm_schedule(&self) -> RealmEventOccurrenceList {
        let realm_id = self.realm_response.as_ref().unwrap().id;
        let start_time = DateTime::parse_from_rfc3339("2024-06-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let end_time = DateTime::parse_from_rfc3339("2024-07-01T00:00:00Z").unwrap().with_timezone(&Utc);
        self
            .authorized_pqry(
                Method::GET,
                &format!("api/realms/{realm_id}/calendar/schedule"),
                OccurrenceQuery {
                    start: start_time,
                    end: end_time
                }
            ).await
    }

    async fn authorized_pst<T: serde::Serialize, R: DeserializeOwned>(&self, method: Method, endpoint: &str, body: &T) -> R {
        let response = self
            .authorized_snd(method, endpoint, body)
            .await;
        parse_body(endpoint, response).await
    }

    async fn authorized_fch<T: DeserializeOwned>(&self, method: Method, endpoint: &str) -> T {
        let response = self
            .authorized_qry(method, endpoint)
            .await;
        parse_body(endpoint, response).await
    }

    async fn authorized_pqry<T: DeserializeOwned, P: Serialize>(&self, method: Method, endpoint: &str, params: P) -> T {
        let response = self.authorized_req(method, endpoint)
            .query(&params)
            .send()
            .await
            .unwrap();
        parse_body(endpoint, response).await
    }

    async fn authorized_qry(&self, method: Method, endpoint: &str) -> Response {
        self.authorized_req(method, endpoint)
            .send()
            .await
            .unwrap()
    }

    async fn authorized_snd<T : Serialize>(&self, method: Method, endpoint: &str, body: &T) -> Response {
        self.authorized_req(method, endpoint)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .expect("Failed to send request")
    }

    #[inline]
    fn authorized_req(&self, method: Method, endpoint: &str) -> RequestBuilder {
        self.client
            .request(method, &format!("{}/{}", self.base_url, endpoint))
            .header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()))
    }
}

async fn parse_body<R : DeserializeOwned>(endpoint: &str, response: Response) -> R {
    if response.status().is_success() {
        let response_body = response.text().await.unwrap();
        let deserialized = serde_json::from_str(&response_body);
        match deserialized {
            Ok(data) => data,
            Err(err) => panic!("Failed to deserialize response at {}: {} for body {}", endpoint, err, response_body)
        }
    } else {
        panic!("Request to {} failed with status {}, body: {}", endpoint, response.status(), response.text().await.unwrap());
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
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

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
    println!("Token: {:?}", context.token.as_ref().unwrap());

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

    let created_realm_event = context.create_realm_event().await;
    println!("Created realm event: {:?}", created_realm_event);

    let schedule = context.get_realm_schedule().await;
    assert!(!schedule.events.is_empty());
    assert!(!schedule.occurrences.is_empty());
    assert_eq!(schedule.occurrences.len(), 12);
    println!("Fetched realm schedule: {:?}", schedule);

    println!("Test complete flow succeeded");
}