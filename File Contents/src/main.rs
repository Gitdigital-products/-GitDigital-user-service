use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserProfile {
    id: String,
    name: String,
    email: String,
    avatar_url: Option<String>,
}

type Db = Arc<Mutex<HashMap<String, UserProfile>>>;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/profile", post(create_profile))
        .route("/profile/:id", get(get_profile))
        .with_state(db.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4400));
    println!("👤 User Profile Service running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_state(db))
        .await
        .unwrap();
}

async fn create_profile(
    State(db): State<Db>,
    Json(input): Json<UserProfile>,
) -> Json<UserProfile> {
    let id = Uuid::new_v4().to_string();
    let profile = UserProfile {
        id: id.clone(),
        name: input.name,
        email: input.email,
        avatar_url: input.avatar_url,
    };

    let mut storage = db.lock().await;
    storage.insert(id.clone(), profile.clone());

    Json(profile)
}

async fn get_profile(Path(id): Path<String>, State(db): State<Db>) -> Json<Option<UserProfile>> {
    let storage = db.lock().await;
    Json(storage.get(&id).cloned())
}
