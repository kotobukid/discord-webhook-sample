mod discord;
mod web;

use crate::discord::run_discord_bot;
use crate::web::run_web_server;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self};

#[tokio::main]
async fn main() {
    // mpscのSenderとReceiverを作成
    let (tx, rx) = mpsc::channel::<String>(32);

    // Webサーバーを動かすスレッドの作成
    let web_server_thread = thread::spawn(move || {
        // tokioランタイムを生成
        let rt = Runtime::new().unwrap();
        // ランタイム内でAxumを非同期実行
        rt.block_on(run_web_server(tx));
    });

    let discord_bot_thread = thread::spawn(move || {
        // tokioランタイムを生成
        let rt = Runtime::new().unwrap();
        // ランタイム内でSerenityを非同期実行
        rt.block_on(run_discord_bot(rx));
    });

    discord_bot_thread.join().unwrap();
    web_server_thread.join().unwrap();
}
