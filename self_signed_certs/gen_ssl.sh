#!/bin/sh

## 秘密鍵の生成
#openssl genrsa -out key.pem 2048
#
## 証明書署名要求(CSR)の作成
#openssl req -new -key key.pem -out csr.pem
#
## 自己署名証明書の生成
#openssl x509 -req -days 365 -in csr.pem -signkey key.pem -out cert.pem

#!/bin/sh

# 出力ファイル名
KEY_FILE="key.pem"
CSR_FILE="csr.pem"
CERT_FILE="cert.pem"

# 証明書詳細情報
SUBJECT="/C=JP/ST=Tokyo/L=Chiyoda/O=YourOrganization/OU=IT/CN=localhost"

# 秘密鍵の生成（RSA 2048ビット、PEMフォーマットを明示指定）
openssl genpkey -algorithm RSA -out "$KEY_FILE" -pkeyopt rsa_keygen_bits:2048
if [ $? -ne 0 ]; then
  echo "秘密鍵の生成に失敗しました。"
  exit 1
fi

# 証明書署名要求 (CSR) の作成
openssl req -new -key "$KEY_FILE" -out "$CSR_FILE" -subj "$SUBJECT"
if [ $? -ne 0 ]; then
  echo "CSRの生成に失敗しました。"
  exit 1
fi

# 自己署名証明書の生成（365日間有効）
openssl x509 -req -days 365 -in "$CSR_FILE" -signkey "$KEY_FILE" -out "$CERT_FILE"
if [ $? -ne 0 ]; then
  echo "自己署名証明書の生成に失敗しました。"
  exit 1
fi

# 成功メッセージ
echo "秘密鍵、CSR、および自己署名証明書が正常に作成されました。"
echo "秘密鍵: $KEY_FILE"
echo "CSR: $CSR_FILE"
echo "証明書: $CERT_FILE"