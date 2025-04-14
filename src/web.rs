use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Deserialize)]
struct HookMessage {
    message: String,
}

async fn webhook_handler(
    State(sender): State<Sender<String>>,
    Json(payload): Json<HookMessage>,
) -> impl IntoResponse {
    // println!("webhook_handler: {:?}", payload);
    let sender = sender.clone();

    if let Err(e) = sender.send(payload.message.clone()).await {
        eprintln!("メッセージの送信に失敗: {}", e);
    }

    (StatusCode::OK, "OK")
}

async fn get_index_handler() -> &'static str {
    "Hello, world!"
}

pub async fn run_web_server(tx: Sender<String>) {
    // Axumルーターの設定
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .route("/", get(get_index_handler))
        .with_state(tx.clone());

    // サーバーを起動
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Webhook Server is listening on {}", addr);
    let socket: std::io::Result<TcpListener> = TcpListener::bind(&addr).await;
    let listener: TcpListener = socket.expect("Failed to bind socket");
    let _ = axum::serve(listener, app.into_make_service()).await;
}
