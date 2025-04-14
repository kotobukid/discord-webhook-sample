use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Timelike};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::num::ParseIntError;
// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Receiver, Sender};

static CHANNEL: OnceCell<u64> = OnceCell::new();

pub fn get_channel() -> Result<u64, ParseIntError> {
    CHANNEL.get().cloned().map(Ok).unwrap_or_else(|| {
        let value = dotenv::var("CHANNEL").unwrap_or_default();
        let parsed = value.parse::<u64>()?;
        CHANNEL.set(parsed).ok();
        Ok(parsed)
    })
}

struct Handler {
    receiver: Receiver<String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!wp" {
            let strings: Result<String, String> = Result::Ok("test".to_string());

            match strings {
                Ok(s) => {
                    let res = msg
                        .channel_id
                        .say(&ctx.http, follow_cutting_string(s))
                        .await;
                    match res {
                        Ok(_) => (),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {
                    eprintln!("get_nearest_events() failed.");
                }
            }
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let id: u64 = get_channel().unwrap();
        let channel_id: ChannelId = ChannelId::from(id);

        tokio::spawn(async move {
            loop {
                let now: DateTime<Local> = Local::now();
                let mut date: NaiveDate = now.date_naive();

                let target_hour: u32 = 0;
                let target_min: u32 = 5;

                if now.time().hour() > target_hour
                    || (now.time().hour() == target_hour && now.time().minute() >= target_min)
                {
                    // If it is past 00:05, set date to tomorrow. 地球が存在する限り失敗しない
                    date = date.succ_opt().expect("failed to get next date");
                }

                // このハードコーディングであれば失敗しない
                let tomorrow_at_0005: NaiveDateTime = date
                    .and_hms_opt(target_hour, target_min, 0)
                    .expect("failed to get time");
                // println!("{}", tomorrow_at_0005.format("%F %R"));

                let duration_to_wait = tomorrow_at_0005.signed_duration_since(now.naive_local());

                println!(
                    "next action at after {} min",
                    duration_to_wait.num_minutes()
                );

                tokio::time::sleep(duration_to_wait.to_std().unwrap()).await;

                let strings: Result<String, String> = Ok("test".to_string());

                match strings {
                    Ok(s) => {
                        let res = channel_id
                            .say(&context.http, follow_cutting_string(s))
                            .await;
                        match res {
                            Ok(_) => (),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    Err(e) => {
                        eprintln!("get_nearest_events() failed.");
                        eprintln!("{:?}", e)
                    }
                }
            }
        });
    }
}

fn get_first_n_characters(s: &str, n: usize) -> CutStringResult {
    let mut chars = s.chars();
    let mut len = 0;
    let mut count = 0;
    for (_i, c) in chars.by_ref().enumerate() {
        len += c.len_utf8();
        count += 1;
        if count == n {
            if s.len() > len {
                return CutStringResult::Cut(s[..len].to_string());
            } else {
                return CutStringResult::Pass(s[..len].to_string());
            }
        }
    }
    if s.len() > len {
        CutStringResult::Cut(s[..len].to_string())
    } else {
        CutStringResult::Pass(s[..len].to_string())
    }
}

enum CutStringResult {
    Pass(String),
    Cut(String),
}

fn follow_cutting_string(string: String) -> String {
    let string_cut = get_first_n_characters(&string, 1900);
    match string_cut {
        CutStringResult::Pass(s) => s,
        CutStringResult::Cut(s) => format!("{}\n(省略されました)", s),
    }
}

async fn run_discord_bot(rx: Receiver<String>) {
    match dotenv::dotenv() {
        Ok(_) => (),
        Err(error) => eprintln!("Couldn't read .env file: {:?}", error),
    }

    let token = match dotenv::var("DISCORD_TOKEN") {
        Ok(value) => value,
        Err(_error) => panic!("Expected a token in the environment"),
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { receiver: rx })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[derive(Debug, Deserialize)]
struct HookMessage {
    message: String,
}

async fn webhook_handler(
    State(sender): State<Sender<String>>,
    Json(payload): Json<HookMessage>,
) -> impl IntoResponse {
    println!("webhook_handler: {:?}", payload);
    let sender = sender.clone();

    if let Err(e) = sender.send(payload.message.clone()).await {
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
        .route("/webhook", post(webhook_handler))
        .route("/", get(get_index_handler))
        .with_state(tx.clone());

    // サーバーを起動
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Webサーバーは {} で起動中...", addr);
    let socket: std::io::Result<TcpListener> = TcpListener::bind(&addr).await;
    let listener: TcpListener = socket.expect("Failed to bind socket");
    let _ = axum::serve(listener, app.into_make_service()).await;
}

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
