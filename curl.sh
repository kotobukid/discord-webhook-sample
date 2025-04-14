#!/bin/bash

# POSTするデータ(JSON形式)
DATA='{"message":"from curl"}'

# HTTPエンドポイント
URL="http://localhost:3000/webhook"

# curlコマンドでデータをPOST
curl -X POST -H "Content-Type: application/json" -d "$DATA" "$URL"