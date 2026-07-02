# Release Checklist

This checklist defines what must be true before presenting Perch as a finished portfolio project.

## Implemented

- Next.js product site with real screenshots in the README
- owner dashboard preview for sites, snippets, indexed pages, and conversations
- standalone widget script served from Gateway at `/widget/perch.js`
- Gateway service for public widget traffic and owner routes
- Indexer service for page ingestion, chunking, and Qdrant upserts
- Retrieval service for tenant-scoped cited answers
- Postgres schema and migrations for tenants, pages, chunks, crawl jobs, conversations, and messages
- Qdrant vector retrieval with Postgres fallback
- Redis configured for the queue-backed worker path
- optional OpenAI-compatible LLM generation
- Docker Compose local stack with service-specific configs
- smoke test that creates a site, indexes a page, verifies Qdrant points, calls widget chat, and checks citations

## Verification Commands

```sh
cargo fmt --all -- --check
cargo check --workspace
cd apps/web
npm install
npm run build
cd ../..
docker compose config
docker compose up --build -d
./scripts/smoke-test.sh
```

## Demo Flow

```sh
docker compose up --build -d
./scripts/smoke-test.sh
./scripts/portfolio-demo.sh
cd apps/web
npm run dev
```

Open:

```txt
http://localhost:3000
http://localhost:3000/dashboard
http://localhost:3000/widget-demo?key=pk_dev_...
```

## Security Notes

- owner routes use `PERCH_OWNER_TOKEN` for local portfolio access
- widget routes use public widget keys and browser origin checks
- widget answers are grounded in indexed tenant pages
- default demo mode disables external LLM calls
- production auth, billing, rate limits, private crawling controls, and secret rotation are intentionally out of scope for this local portfolio build

## Portfolio Review Checklist

- README explains the product in under two minutes
- README shows the tech stack, architecture, demo commands, and intentional limits
- screenshots are distinct and current
- Docker stack starts from a clean checkout
- smoke test passes
- dashboard communicates connected, preview, empty, and indexed states
- widget can be embedded through a script tag and handles loading/error states
- docs do not claim production readiness where the code only provides local portfolio readiness
