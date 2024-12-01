use std::{collections::HashMap, sync::{Arc, Mutex}};

use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, http::HeaderMap, response::IntoResponse, routing::{get, Router}};
use futures::{SinkExt, StreamExt};
use tower_http::services::ServeFile;
use uuid::Uuid;

use quoridor_platform::game::Game;

struct AppState {
    games: Mutex<HashMap<Uuid, Game>>
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        games: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/ws", get(handler))
        .with_state(state); 

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap()); 
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("{:?}", headers);
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, _state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    loop {
        if let Some(Ok(Message::Text(msg))) = receiver.next().await {
            sender
                .send(Message::from(format!("Hello {:?}", msg)))
                .await
                .expect("Can't send response");
        }
    }
}

