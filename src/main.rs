use std::net::SocketAddr;

use axum::{extract::{ws::{WebSocket}, Path, State, WebSocketUpgrade}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::{pool, types::time::PrimitiveDateTime, PgPool};
use chrono::NaiveDateTime;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Serialize)]
struct Message {
    id: u32,
    sender_id: Option<u32>,
    receiver_id: u32,
    content: String,
    created_at: PrimitiveDateTime,
    edited_at: Option<PrimitiveDateTime>,
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
        .route("/users/{name}", get(get_user))
        .route("/insert_john", get(insert_users))
        .route("/msgs", get(get_msgs))
        .route("/insert_msg", get(insert_msgs))
        .route("/ws", get(ws_handler))
        .with_state(pool); // attach pool as shared state

    let addr = SocketAddr::from(([0, 0, 0, 0], 1337));
    println!("Server running on {addr}");
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket);
}

async fn handle_socket(mut socket: WebSocket) {
    println!("New websocket connection!");

    let socket_stts = socket.send(axum::extract::ws::Message::Text("Hello dude!".to_string().into())).await.is_err();
    if socket_stts {
        println!("Client disconnected early");
        return;
    }
}

async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query!("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .expect("Couldnt get users");

    let vec_users: Vec<User> = users.iter().map(|a| User { id: a.id as u32, name: a.name.clone()}).collect();
    Json(vec_users)
}

async fn get_user(
    Path(name): Path<String>,
    State(pool): State<PgPool>
) -> Json<User> {
    let rec_user = sqlx::query!("SELECT id, name FROM users WHERE name = $1", name)
        .fetch_one(&pool)
        .await
        .expect("Couldnt get users");

    let user = User { id: rec_user.id as u32, name: rec_user.name.clone() };
    Json(user)
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

async fn get_msgs(State(pool): State<PgPool>) -> Json<Vec<Message>> {
    let msgs = sqlx::query!("SELECT id, content, sender_id, receiver_id, created_at, edited_at FROM messages")
        .fetch_all(&pool)
        .await
        .expect("Couldnt get msgs");

    let vec_msgs: Vec<Message> = msgs.iter()
        .map(|msg| Message {
            id: msg.id as u32,
            content: msg.content.clone().unwrap_or("".to_string()),
            sender_id: Some(msg.sender_id.unwrap_or(0) as u32),
            receiver_id: msg.receiver_id.unwrap_or(0) as u32,
            created_at: msg.created_at.unwrap(),
            edited_at: msg.edited_at,
        }).collect();
    Json(vec_msgs)
}

// async fn get_msg(
//     Path(msg_id): Path<String>,
//     State(pool): State<PgPool>
// ) -> Json<msg> {
//     let rec_msg = sqlx::query!("SELECT id, name FROM messages WHERE id = $1", msg_id)
//         .fetch_one(&pool)
//         .await
//         .expect("Couldnt get msgs");
//
//     let msg = User { id: rec_msg.id as u32, name: rec_msg.name.clone() };
//     Json(msg)
// }

async fn insert_msgs(State(pool): State<PgPool>) -> impl IntoResponse {
    let content = "hello world";
    let result = sqlx::query!(
        "INSERT INTO messages (content) VALUES ($1)",
        content
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, "msg inserted").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert msg").into_response()
    }
}

