#!/bin/sh

sleep 5

hydra clients create \
  --endpoint http://hydra:4445 \
  --id my-client \
  --secret my-secret \
  --grant-types authorization_code,refresh_token \
  --response-types code \
  --scope openid,profile,email,offline \
  --callbacks http://localhost:8080/callback \
  --token-endpoint-auth-method client_secret_basic \
  --skip-tls-verify
