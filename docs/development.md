# Development

## Requirements

- Node.js 20 or newer
- npm
- Rust toolchain
- Docker

## Web App

Install dependencies:

```sh
cd apps/web
npm install
```

Run locally:

```sh
npm run dev
```

Open the dashboard preview:

```txt
http://localhost:3000/dashboard
```

Build:

```sh
npm run build
```

## Rust Backend

Check the full Rust workspace:

```sh
cargo check --workspace
```

Format Rust code:

```sh
cargo fmt --all
```

Run a service:

```sh
cargo run -p perch-gateway
```

Service bind addresses can be overridden with:

```sh
GATEWAY_BIND_ADDR=127.0.0.1:8080
INDEXER_BIND_ADDR=127.0.0.1:8081
RETRIEVAL_BIND_ADDR=127.0.0.1:8082
```

Shared backend environment variables:

```sh
PERCH_ENV=development
PERCH_DATABASE_URL=postgres://perch:perch@127.0.0.1:5433/perch
PERCH_REDIS_URL=redis://127.0.0.1:6380
PERCH_QDRANT_URL=http://127.0.0.1:6335
PERCH_INDEXER_URL=http://127.0.0.1:8081
PERCH_RETRIEVAL_URL=http://127.0.0.1:8082
```

Health endpoints:

```txt
gateway    http://localhost:18080/health
indexer    http://localhost:18081/health
retrieval  http://localhost:18082/health
```

Readiness endpoints expose dependency readiness:

```txt
gateway    http://localhost:18080/ready
indexer    http://localhost:18081/ready
retrieval  http://localhost:18082/ready
```

`/ready` returns HTTP 503 when a required dependency cannot be reached. Gateway requires Postgres. Indexer and Retrieval require Postgres plus Qdrant when `PERCH_QDRANT_ENABLED=true`; disabled Qdrant is reported as `configured`.

## Local Infrastructure

Start the Tier A local stack:

```sh
docker compose up --build
```

Postgres migrations run through the `migrate` service before backend services start.

Run in the background:

```sh
docker compose up --build -d
```

Check service state:

```sh
docker compose ps
```

Run the end-to-end smoke test:

```sh
./scripts/smoke-test.sh
```

Run migrations only:

```sh
docker compose up migrate
```

Stop the stack:

```sh
docker compose down
```

The local stack exposes:

```txt
gateway    http://localhost:18080/health
indexer    http://localhost:18081/health
retrieval  http://localhost:18082/health
gateway    http://localhost:18080/ready
indexer    http://localhost:18081/ready
retrieval  http://localhost:18082/ready
postgres   localhost:5433
redis      localhost:6380
qdrant     http://localhost:6335/readyz
```

Local container service URLs use Docker network hostnames:

```txt
postgres://perch:perch@postgres:5432/perch
redis://redis:6379
http://qdrant:6333
```

Database migrations live in:

```txt
infra/database/migrations
```

## Environment

Start from `.env.example` files and keep secrets out of Git.

Never commit:

- provider API keys
- database URLs with passwords
- session secrets
- production widget keys
- crawl targets containing private customer data

## Quality Bar

Before merging visible UI work:

- build the app
- verify desktop and mobile layout
- check that product copy says Perch, not another project
- include screenshots in the pull request

Before merging backend work:

- run formatting
- run tests
- document new environment variables
- update architecture docs when boundaries change

## Gateway Bootstrap

Create a local site:

```sh
curl -X POST http://localhost:18080/v1/sites \
  -H 'content-type: application/json' \
  -d '{"organization_name":"Acme","site_name":"Acme Docs","origin":"https://docs.acme.example"}'
```

Fetch widget config:

```sh
curl 'http://localhost:18080/v1/widget/config?key=pk_dev_...' \
  -H 'origin: https://docs.acme.example'
```

Send a widget chat message:

```sh
curl -X POST http://localhost:18080/v1/widget/chat \
  -H 'content-type: application/json' \
  -H 'origin: https://docs.acme.example' \
  -d '{"public_key":"pk_dev_...","session_id":"local-session","message":"How do I install Perch?"}'
```

Retrieval reads from `page_chunks`. Until the indexer writes real chunks, responses fall back to a clear no-indexed-context message.

Index a page through Gateway:

```sh
curl -X POST http://localhost:18080/v1/sites/.../pages \
  -H 'content-type: application/json' \
  -d '{"url":"https://docs.acme.example/install","title":"Install Perch","content":"Install Perch with one script tag.","content_type":"text"}'
```

Crawl and index one page through Gateway:

```sh
curl -X POST http://localhost:18080/v1/sites/.../crawl-jobs \
  -H 'content-type: application/json' \
  -d '{"url":"https://docs.acme.example/install"}'
```

Read crawl job status:

```sh
curl http://localhost:18080/v1/sites/.../crawl-jobs/...
```

Connect the Next.js demo widget to Gateway:

```sh
NEXT_PUBLIC_PERCH_GATEWAY_URL=http://localhost:18080
NEXT_PUBLIC_PERCH_WIDGET_KEY=pk_dev_...
```
