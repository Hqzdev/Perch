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

Health endpoints:

```txt
http://localhost:18080/health
http://localhost:18081/health
http://localhost:18082/health
```

## Local Infrastructure

Start the Tier A local stack:

```sh
docker compose up --build
```

Run in the background:

```sh
docker compose up --build -d
```

Check service state:

```sh
docker compose ps
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
