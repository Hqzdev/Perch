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

This is a bootstrap JSON endpoint. Streaming and retrieval-backed answers are the next backend stage.

## Dashboard

Dashboard routes require authenticated sessions.

Implemented bootstrap routes:

- `POST /v1/sites`

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
