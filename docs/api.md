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

### Stream Answer

```txt
POST /v1/widget/chat
Origin: https://customer-site.example
```

Request:

```json
{
  "publicKey": "pk_live_...",
  "sessionId": "session_...",
  "message": "How do I reset my API key?"
}
```

Response uses server-sent events:

```txt
event: token
data: {"text":"Open Settings"}

event: citation
data: {"title":"API Keys","url":"https://acme.example/docs/api-keys"}

event: done
data: {}
```

## Dashboard

Dashboard routes require authenticated sessions.

Planned routes:

- `POST /v1/sites`
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
