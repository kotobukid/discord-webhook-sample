use std::sync::{Arc, Mutex};
use axum::{
    routing::{get, post},
    Json, Router,
};
use std::sync::mpsc::{self, Sender};
use std::thread;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Message {
    message: String,
}

async fn webhook_handler(State(sender): State<Sender<String>>, Json(payload): Json<Message>) -> impl IntoResponse {
    println!("webhook_handler: {:?}", payload);
    let sender = sender.clone();

    if let Err(e) = sender.send(payload.message.clone()) {
        eprintln!("メッセージの送信に失敗: {}", e);
    }

    (StatusCode::OK, "OK")
}

async fn get_index_handler() -> &'static str {
    "Hello, world!"
}

async fn run_web_server(tx: Sender<String>) {
    // Axumルーターの設定
    let app = Router::new()
        .route(
            "/webhook",
            post(webhook_handler),
        )
        .route(
            "/",
            get(get_index_handler),
        ).with_state(tx.clone());

    // サーバーを起動
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Webサーバーは {} で起動中...", addr);
    let socket: std::io::Result<TcpListener> = TcpListener::bind(&addr).await;
    let listener: TcpListener = socket.expect("Failed to bind socket");
    let _ = axum::serve(listener, app.into_make_service()).await;
}

fn main() {
    // mpscのSenderとReceiverを作成
    let (tx, rx) = mpsc::channel::<String>();

    // Webサーバーを動かすスレッドの作成
    let web_server_thread = thread::spawn(move || {
        // tokioランタイムを生成
        let rt = Runtime::new().unwrap();
        // ランタイム内でAxumを非同期実行
        rt.block_on(run_web_server(tx));
    });

    // メインスレッドではmpscのReceiverを使った処理を行う
    loop {
        match rx.recv() {
            Ok(message) => {
                println!("受信したメッセージ: {}", message);
                // ここでメッセージを処理するコードを追加
            }
            Err(e) => {
                eprintln!("チャネルエラー: {}", e);
                break;
            }
        }
    }

    // Webサーバースレッドの終了を待つ
    web_server_thread.join().unwrap();
}
