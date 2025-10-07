use std::net::SocketAddr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::{pool, PgPool};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("Starting Yapp's backend..");

    dotenv().ok(); // load .env

    // postgres connection pool
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    let pool = PgPool::connect(&url).await.expect("Failed to connect to database");

    // router
    let app = Router::new()
        .route("/users", get(get_users))
        .route("/insert_john", get(insert_users))
        .with_state(pool); // attach pool as shared state

    let addr = SocketAddr::from(([0, 0, 0, 0], 1337));
    println!("Server running on {addr}");
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query!("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .expect("Couldnt get users");

    let vec_users: Vec<User> = users.iter().map(|a| User { id: a.id as u32, name: a.name.clone()}).collect();
    Json(vec_users)
}

async fn insert_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let name = "John Doe";
    let password = "0cbc6611f5540bd0809a388dc95a615b"; // "Test" in md5
    let result = sqlx::query!(
        "INSERT INTO users (name, password) VALUES ($1, $2)",
        name,
        password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, "User inserted").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert user").into_response()
    }
}
