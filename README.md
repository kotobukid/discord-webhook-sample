# Webhook to Discord 通知サンプル

このプロジェクトは、AxumウェブサーバーとSerenity Discord botを使用して、WebhookからDiscordチャンネルへ通知を送信するサンプルアプリケーションです。

## 機能

- Axumを使用したWebhookエンドポイントの提供
- 受信したWebhookメッセージをDiscordチャンネルに転送
- 長いメッセージの自動切り詰め

## 必要条件

- Rust (2021 Edition)
- Discordボットトークン
- Discordチャンネル ID

## インストール方法

1. リポジトリをクローンします：

```bash
git clone https://github.com/kotobukid/discord-webhook-sample
cd discord-webhook-sample
```

2. 環境変数を設定します：

`.env`ファイルを作成し、以下の内容を設定します：

```
DISCORD_TOKEN=あなたのDiscordボットトークン
CHANNEL=通知を送信するDiscordチャンネルID
```

## 使用方法

### サーバーの起動

```bash
cargo run
```

サーバーは`0.0.0.0:3000`でリッスンを開始します。

### Webhookへのメッセージ送信

#### curlを使用する場合：

```bash
curl -X POST -H "Content-Type: application/json" -d '{"message":"こんにちは、世界！"}' http://localhost:3000/webhook
```

#### HTTPクライアントを使用する場合：

```http
POST http://127.0.0.1:3000/webhook
Content-Type: application/json

{"message": "こんにちは、世界！"}
```

## 仕組み

1. Axumウェブサーバーが`/webhook`エンドポイントでPOSTリクエストを受け付けます
2. 受信したメッセージはチャネルを通じてDiscordボットに転送されます
3. Discordボットは設定されたチャンネルにメッセージを送信します

## プロジェクト構造

- `src/main.rs` - メインアプリケーションのエントリーポイント
- `src/web.rs` - Axumウェブサーバーの実装
- `src/discord.rs` - Serenity Discordボットの実装
- `sample.env` - 環境変数のサンプル
- `sample.http` - HTTPリクエストのサンプル
- `curl.sh` - curlコマンドのサンプルスクリプト

## ライセンス

MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.