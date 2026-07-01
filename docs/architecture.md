# Architecture

Perch is a three-service system with clean internal boundaries. The service split is based on workload shape, not microservice fashion.

## Services

### Gateway

Gateway owns the public edge.

Responsibilities:

- widget script and configuration endpoints
- public widget key validation
- dashboard session authentication
- tenant lookup
- domain allow-list enforcement
- rate limiting
- forwarding chat streams to retrieval
- starting indexing jobs

Gateway must not own crawling, embedding, reranking, or answer generation logic.

### Indexer

Indexer owns website ingestion.

Responsibilities:

- crawl jobs
- robots.txt policy
- sitemap discovery
- URL normalization
- HTML fetching
- readable text extraction
- chunking
- embedding
- vector upsert
- page and chunk metadata persistence

Indexer must not serve visitor chat traffic.

### Retrieval

Retrieval owns question answering.

Responsibilities:

- question embedding
- vector search
- keyword search when available
- reranking
- context assembly
- prompt assembly
- LLM streaming
- citation mapping

Retrieval must not bypass tenant filters.

## Internal Structure

Each Rust service should use:

```txt
src/
  domain/
  application/
  infrastructure/
  interfaces/
  config/
  main.rs
```

### Domain

Domain contains entities, value objects, and rules.

Domain must not depend on:

- HTTP frameworks
- SQL libraries
- Redis
- Qdrant
- external model providers
- environment variables

### Application

Application contains use cases. It coordinates domain rules and calls ports.

Examples:

- `start_crawl`
- `index_page`
- `answer_question`
- `resolve_widget_config`
- `validate_embed_origin`

### Infrastructure

Infrastructure contains adapters for external systems.

Examples:

- Postgres repositories
- Redis queues
- Qdrant vector index
- HTTP crawler
- embedding clients
- rerank clients
- LLM clients

### Interfaces

Interfaces contain entrypoints.

Examples:

- HTTP handlers
- queue consumers
- health endpoints

## Shared Crates

### rag-core

Pure RAG logic only:

- chunking
- source spans
- citation data structures
- retrieved context types
- embedding vector types
- normalized URL values

It must not call Postgres, Redis, Qdrant, OpenAI, Anthropic, Cohere, or HTTP APIs.

### perch-types

Shared contracts:

- API DTOs
- events
- identifiers
- tenant-safe IDs

### perch-config

Shared configuration loading only when duplication becomes real.

### perch-storage

Shared storage adapters:

- Postgres pool ownership
- database readiness checks
- shared repository primitives when duplication becomes real

Services may depend on `perch-storage`; domain modules must not.

## Data Ownership

Postgres is the source of truth for tenants, sites, pages, jobs, widget settings, usage, and metadata.

Qdrant is a retrieval index, not the source of truth.

Redis is operational state, not durable business state.

Raw HTML snapshots may later live in object storage.

## Boundary Rules

Allowed:

```txt
gateway -> retrieval HTTP
gateway -> indexer HTTP or queue
indexer -> rag-core
retrieval -> rag-core
services -> perch-types
services -> perch-config
```

Forbidden:

```txt
gateway imports indexer internals
retrieval imports gateway internals
rag-core calls model providers
domain imports sqlx
domain imports axum
domain imports qdrant-client
widget receives trusted tenant_id from browser
```

## V1 Data Flow

```txt
site URL
  -> gateway
  -> crawl job
  -> indexer
  -> fetch pages
  -> extract text
  -> chunk
  -> embed
  -> Postgres metadata
  -> Qdrant vectors

visitor question
  -> widget
  -> gateway
  -> retrieval
  -> embed question
  -> tenant-filtered search
  -> rerank
  -> context
  -> LLM stream
  -> cited answer
  -> widget
```
