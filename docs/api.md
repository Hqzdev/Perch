# API Design

This document captures intended API contracts. Exact routes can change before the backend is implemented.

## Service Operations

All backend services expose health and readiness endpoints.

```txt
GET /health
```

Returns:

```json
{
  "service": "gateway",
  "status": "ok"
}
```

```txt
GET /ready
```

Returns:

```json
{
  "service": "gateway",
  "status": "ok",
  "environment": "development",
  "dependencies": [
    { "name": "postgres", "status": "ok" },
    { "name": "redis", "status": "configured" },
    { "name": "qdrant", "status": "configured" }
  ]
}
```

When Postgres is unavailable, `/ready` returns HTTP 503 and marks the service as `unavailable`.

## Public Widget

### Get Widget Configuration

```txt
GET /v1/widget/config?key=pk_live_...
Origin: https://customer-site.example
```

Returns:

```json
{
  "siteName": "Acme Docs",
  "theme": {
    "accentColor": "#12b76a",
    "placement": "bottom-right"
  },
  "features": {
    "citations": true,
    "streaming": true
  }
}
```

The server resolves tenant and site from the public key. The browser must not provide trusted tenant IDs.

The request must include an `Origin` header matching the registered site origin. A mismatched origin returns:

```json
{
  "error": {
    "code": "domain_not_allowed",
    "message": "This widget key is not allowed on the current domain."
  }
}
```

### Answer Message

```txt
POST /v1/widget/chat
Origin: https://customer-site.example
```

Request:

```json
{
  "public_key": "pk_live_...",
  "session_id": "session_...",
  "message": "How do I reset my API key?"
}
```

Returns:

```json
{
  "conversation_id": "018f0000-0000-7000-9000-000000000003",
  "message_id": "018f0000-0000-7000-9000-000000000004",
  "answer": "Perch received your question for Acme Docs and stored it for retrieval...",
  "citations": [
    {
      "title": "Acme Docs",
      "url": "https://docs.acme.example"
    }
  ]
}
```

Gateway validates the widget key and origin, then calls Retrieval through `POST /v1/answer`. Retrieval currently searches Postgres `page_chunks` with a simple keyword lookup. Qdrant vector search and streaming are the next backend stage.

## Internal Retrieval

```txt
POST /v1/answer
```

Request:

```json
{
  "site_id": "018f0000-0000-7000-9000-000000000001",
  "site_name": "Acme Docs",
  "site_origin": "https://docs.acme.example",
  "question": "How do I reset my API key?"
}
```

Returns:

```json
{
  "answer": "Based on indexed pages for Acme Docs, the closest matching source says...",
  "citations": [
    {
      "title": "Acme Docs",
      "url": "https://docs.acme.example"
    }
  ]
}
```

## Page Indexing

Gateway route:

```txt
POST /v1/sites/:siteId/pages
```

It validates that the site exists, then forwards the page to Indexer.

Internal Indexer route:

```txt
POST /v1/index/pages
```

Request:

```json
{
  "site_id": "018f0000-0000-7000-9000-000000000001",
  "url": "https://docs.acme.example/install",
  "title": "Install Perch",
  "content": "<main>Install Perch with one script tag...</main>",
  "content_type": "html"
}
```

Returns:

```json
{
  "page_id": "018f0000-0000-7000-9000-000000000005",
  "chunks_indexed": 3
}
```

The current endpoint is a direct ingestion API. Crawling, sitemap discovery, robots policy, and queue-based jobs remain separate indexer work.

## Dashboard

Dashboard routes require authenticated sessions.

Implemented bootstrap routes:

- `POST /v1/sites`
- `POST /v1/sites/:siteId/pages`

Create site request:

```json
{
  "organization_name": "Acme",
  "site_name": "Acme Docs",
  "origin": "https://docs.acme.example"
}
```

Create site response:

```json
{
  "id": "018f0000-0000-7000-9000-000000000001",
  "organization_id": "018f0000-0000-7000-9000-000000000002",
  "name": "Acme Docs",
  "origin": "https://docs.acme.example",
  "script_key": "pk_dev_..."
}
```

Planned routes:

- `GET /v1/sites`
- `GET /v1/sites/:siteId`
- `POST /v1/sites/:siteId/crawl-jobs`
- `GET /v1/sites/:siteId/crawl-jobs/:jobId`
- `GET /v1/sites/:siteId/questions`
- `PATCH /v1/sites/:siteId/widget-config`

## Error Shape

```json
{
  "error": {
    "code": "domain_not_allowed",
    "message": "This widget key is not allowed on the current domain."
  }
}
```

Error messages should be useful without leaking tenant existence or private configuration.
