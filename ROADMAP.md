# Roadmap

Perch should be built in tiers. The project loses focus if Tier B infrastructure work starts before Tier A proves the product.

## Tier A

Goal: a working website assistant demo.

- Next.js website and dashboard preview
- framework-free embeddable widget
- Rust gateway service
- Rust indexer service
- Rust retrieval service
- Postgres for tenants, sites, pages, chunks, jobs, and usage events
- Redis for queue and rate limiting
- Qdrant for vector search
- external embedding provider
- external rerank provider
- external LLM provider
- crawl one public website
- index HTML pages
- ask questions through the widget
- stream answers with citations
- enforce tenant isolation
- enforce domain allow-listing
- local Docker Compose
- web build CI

## Tier B

Goal: make Perch stronger as infrastructure.

- hybrid keyword and vector search
- better citation span mapping
- scheduled reindexing
- dashboard question analytics
- source coverage gaps
- usage and cost ledger
- self-host deployment guide
- OpenTelemetry tracing
- Prometheus metrics
- Grafana dashboard
- load testing with realistic widget traffic
- optional local embedding inference
- optional local reranker inference

## Tier C

Goal: only after product traction.

- multi-region deployment
- autoscaling deployment templates
- custom persistent ANN index
- advanced crawler scheduling
- billing
- agency client management
- enterprise SSO

Tier C should not block the first public demo.
