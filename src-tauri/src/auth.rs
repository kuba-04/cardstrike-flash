use std::sync::Arc;
use tauri::async_runtime::Mutex;
use serde::{Deserialize, Serialize};
use supabase_auth::models::AuthClient;
use uuid::Uuid;

pub struct Auth {
    auth_client: AuthClient,
}

impl Auth {
    pub fn new() -> Auth {
        let auth_client = AuthClient::new_from_env().unwrap();
        Auth { auth_client }
    }

    pub async fn login(&self, email: &str, password: &str) -> supabase_auth::models::Session {
        self.auth_client
            .login_with_email(email, password)
            .await
            .unwrap()
    }
}

#[derive(Clone)]
pub struct AppState(pub Arc<Mutex<Option<Session>>>);

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,  // Timestamp
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
}