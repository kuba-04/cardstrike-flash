use crate::auth::{AppState, Auth, Session, User};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use supabase_rs::SupabaseClient;
use tauri::async_runtime::Mutex;
use tauri::State;

mod auth;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn call_api() {
    println!("{}", "api call api".to_string());
}

#[tauri::command]
async fn login(email: String, password: String, state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let auth_client = Auth::new();
    let sb_session = auth_client.login(&email, &password).await.clone();

    let session = Session {
        access_token: sb_session.access_token,
        refresh_token: sb_session.refresh_token,
        expires_at: sb_session.expires_at.cast_signed(),
        user: User {
            id: sb_session.user.id,
            email: Some(sb_session.user.email),
        },
    };
    let mut app_state = state.0.lock().await;
    *app_state = Some(session.clone());

    Ok(serde_json::json!({ "user": session.user, "success": true }))
}

#[tauri::command]
async fn get_cards(state: State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    dotenv().ok();
    let app_state = state.0.lock();
    let session = app_state.await.clone().ok_or(String::from("No session"))?;

    let client = SupabaseClient::new(
        env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    ).unwrap();

    let cards = client
        .select("flashcards")
        .eq("user_id", session.user.id.to_string().as_str())
        .execute()
        .await?;
    Ok(cards)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Arc::new(Mutex::new(None))))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, call_api, login, get_cards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
//
//
// struct FlashCard {
//     id: String,
//     user_id: String,
//     front_content: String,
//     back_content: String,
//     next_review_at: UtcDateTime,
//     collection_id: String,
// }
