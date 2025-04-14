use axum::extract::State;
use axum::http::{Method, StatusCode};
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use std::time::Duration;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tower_http::cors::{Any, CorsLayer};
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

pub async fn cors_handler() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![
            // Method::GET,
            Method::POST,
            // Method::PUT,
            // Method::DELETE,
            // Method::OPTIONS,
        ])
        .allow_headers(Any)
        // .allow_credentials(true)
        .max_age(Duration::from_secs(86400)) // 1日間のプリフライトキャッシュ
}

pub async fn run_web_server(tx: Sender<String>) {

    let cors = cors_handler().await;

    // Axumルーターの設定
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .route("/", get(get_index_handler))
        .layer(cors)
        .with_state(tx.clone());

    // サーバーを起動
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Webhook Server is listening on {}", addr);
    let socket: std::io::Result<TcpListener> = TcpListener::bind(&addr).await;
    let listener: TcpListener = socket.expect("Failed to bind socket");
    let _ = axum::serve(listener, app.into_make_service()).await;
}
