use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post, Router},
};
use futures::{SinkExt, StreamExt};
use tower_http::services::ServeFile;
use uuid::Uuid;

use quoridor_platform::{
    game::Game,
    player::{AnonUser, PlayerType},
};

struct AppState {
    games: Mutex<HashMap<Uuid, Game>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        games: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/api/v1/new_game", post(new_game))
        .route("/ws", get(websocket_start))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn new_game(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    // Need a way to start off the game with only 1 player known, also it is possible that players
    // are not users but maybe just anonymous
    let player_1 = AnonUser::new());
    state
        .games
        .lock()
        .unwrap()
        .insert(Uuid::new_v4(), Game::new(player_1, player_2, time_control));
}

async fn websocket_start(
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
