#!/usr/bin/env bash
set -euo pipefail

GATEWAY_URL="${GATEWAY_URL:-http://localhost:18080}"

json_field() {
  node -e "const data = JSON.parse(process.argv[1]); console.log(data[process.argv[2]] ?? '')" "$1" "$2"
}

post_json() {
  curl -fsS "$1" \
    -H "content-type: application/json" \
    -d "$2"
}

site_response="$(post_json "$GATEWAY_URL/v1/sites" '{"organization_name":"Portfolio Demo","site_name":"Perch Portfolio Demo","origin":"https://portfolio-demo.perch.local"}')"
site_id="$(json_field "$site_response" id)"
script_key="$(json_field "$site_response" script_key)"

echo "Created site: $site_id"
echo "Widget key: $script_key"

page_response="$(post_json "$GATEWAY_URL/v1/sites/$site_id/pages" '{"url":"https://portfolio-demo.perch.local/docs/install","title":"Install Perch","content":"Perch installs with one script tag. The gateway validates the widget key and origin, the indexer stores page chunks, and retrieval answers visitor questions with citations from indexed source pages.","content_type":"text"}')"
page_id="$(json_field "$page_response" page_id)"
chunks_indexed="$(json_field "$page_response" chunks_indexed)"

echo "Indexed page: $page_id"
echo "Chunks indexed: $chunks_indexed"

chat_response="$(curl -fsS "$GATEWAY_URL/v1/widget/chat" \
  -H "content-type: application/json" \
  -H "origin: https://portfolio-demo.perch.local" \
  -d "{\"public_key\":\"$script_key\",\"session_id\":\"portfolio-demo\",\"message\":\"How does Perch answer with citations?\"}")"

echo "Answer:"
json_field "$chat_response" answer

echo "Citations:"
node -e "const data = JSON.parse(process.argv[1]); for (const citation of data.citations) console.log(\`\${citation.title} -> \${citation.url}\`)" "$chat_response"
