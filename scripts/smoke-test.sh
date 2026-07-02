#!/usr/bin/env bash
set -euo pipefail

GATEWAY_URL="${GATEWAY_URL:-http://localhost:18080}"
INDEXER_URL="${INDEXER_URL:-http://localhost:18081}"
RETRIEVAL_URL="${RETRIEVAL_URL:-http://localhost:18082}"
QDRANT_URL="${QDRANT_URL:-http://localhost:6335}"
QDRANT_COLLECTION="${QDRANT_COLLECTION:-perch_chunks}"
OWNER_TOKEN="${OWNER_TOKEN:-perch_dev_owner_token}"
RUN_ID="${RUN_ID:-$(date +%s)}"
ORIGIN="https://smoke-$RUN_ID.perch.local"

json_field() {
  node -e "const data = JSON.parse(process.argv[1]); console.log(data[process.argv[2]] ?? '')" "$1" "$2"
}

json_path() {
  node -e "const data = JSON.parse(process.argv[1]); const path = process.argv[2].split('.'); let value = data; for (const key of path) value = value?.[key]; console.log(value ?? '')" "$1" "$2"
}

assert_json_field() {
  node -e "const data = JSON.parse(process.argv[1]); const value = data[process.argv[2]]; if (value === undefined || value === null || value === '') process.exit(1)" "$1" "$2"
}

assert_text_contains() {
  node -e "if (!process.argv[1].includes(process.argv[2])) process.exit(1)" "$1" "$2"
}

post_json() {
  curl -fsS "$1" \
    -H "content-type: application/json" \
    -H "x-perch-owner-token: $OWNER_TOKEN" \
    -d "$2"
}

curl -fsS "$GATEWAY_URL/ready" >/dev/null
curl -fsS "$INDEXER_URL/ready" >/dev/null
curl -fsS "$RETRIEVAL_URL/ready" >/dev/null
curl -fsS "$QDRANT_URL/readyz" >/dev/null
node -e "if (process.argv[1] !== '401') process.exit(1)" "$(curl -s -o /dev/null -w "%{http_code}" "$GATEWAY_URL/v1/sites")"

site_response="$(post_json "$GATEWAY_URL/v1/sites" "{\"organization_name\":\"Smoke Demo $RUN_ID\",\"site_name\":\"Perch Smoke Demo $RUN_ID\",\"origin\":\"$ORIGIN\"}")"
assert_json_field "$site_response" id
assert_json_field "$site_response" script_key

site_id="$(json_field "$site_response" id)"
script_key="$(json_field "$site_response" script_key)"

page_response="$(post_json "$GATEWAY_URL/v1/sites/$site_id/pages" "{\"url\":\"$ORIGIN/docs/install\",\"title\":\"Install Perch\",\"content\":\"Perch installs with one script tag. The gateway validates the widget key and origin, the indexer stores page chunks, Qdrant stores vectors, and retrieval answers visitor questions with citations from indexed source pages.\",\"content_type\":\"text\"}")"
assert_json_field "$page_response" page_id
assert_json_field "$page_response" chunks_indexed

chunks_indexed="$(json_field "$page_response" chunks_indexed)"
node -e "if (Number(process.argv[1]) < 1) process.exit(1)" "$chunks_indexed"

count_response="$(curl -fsS "$QDRANT_URL/collections/$QDRANT_COLLECTION/points/count" \
  -H "content-type: application/json" \
  -d "{\"exact\":true}")"
point_count="$(json_path "$count_response" result.count)"
node -e "if (Number(process.argv[1]) < 1) process.exit(1)" "$point_count"

chat_response="$(curl -fsS "$GATEWAY_URL/v1/widget/chat" \
  -H "content-type: application/json" \
  -H "origin: $ORIGIN" \
  -d "{\"public_key\":\"$script_key\",\"session_id\":\"smoke-$RUN_ID\",\"message\":\"How does Perch answer with citations?\"}")"
assert_json_field "$chat_response" answer
assert_text_contains "$chat_response" "$ORIGIN/docs/install"

sites_response="$(curl -fsS "$GATEWAY_URL/v1/sites" -H "x-perch-owner-token: $OWNER_TOKEN")"
assert_text_contains "$sites_response" "$site_id"

site_detail_response="$(curl -fsS "$GATEWAY_URL/v1/sites/$site_id" -H "x-perch-owner-token: $OWNER_TOKEN")"
assert_text_contains "$site_detail_response" "$script_key"
assert_text_contains "$site_detail_response" "data-perch-key"

pages_response="$(curl -fsS "$GATEWAY_URL/v1/sites/$site_id/pages" -H "x-perch-owner-token: $OWNER_TOKEN")"
assert_text_contains "$pages_response" "$ORIGIN/docs/install"

conversations_response="$(curl -fsS "$GATEWAY_URL/v1/sites/$site_id/conversations" -H "x-perch-owner-token: $OWNER_TOKEN")"
assert_text_contains "$conversations_response" "smoke-$RUN_ID"

echo "Smoke test passed"
echo "Site: $site_id"
echo "Chunks indexed: $chunks_indexed"
echo "Qdrant points: $point_count"
