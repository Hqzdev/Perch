# Deployment

Perch does not have a production deployment target yet. This document defines the intended deployment shape so implementation does not drift.

## Local Tier A Deployment

V1 starts with Docker Compose:

- gateway container
- indexer container
- retrieval container
- Postgres
- Redis
- Qdrant
- web app

This is enough for a real demo and avoids premature Kubernetes work.

Run the local stack:

```sh
docker compose up --build
```

Services:

```txt
gateway    http://localhost:18080/health
indexer    http://localhost:18081/health
retrieval  http://localhost:18082/health
postgres   localhost:5433
redis      localhost:6380
qdrant     http://localhost:6335/readyz
```

Persistent local volumes:

```txt
postgres-data
redis-data
qdrant-data
```

Reset local infrastructure state:

```sh
docker compose down --volumes
```

This deletes local data. Use it only when local state can be discarded.

## Required Configuration

- public web URL
- gateway public URL
- widget CDN or static asset URL
- Postgres URL
- Redis URL
- Qdrant URL
- embedding provider key
- rerank provider key
- LLM provider key
- dashboard session secret
- allowed dashboard origin

## Production Concerns

Before calling any deployment production-ready:

- enforce HTTPS
- enforce tenant filters on every retrieval query
- enforce domain allow-listing for widget use
- set crawler timeouts and page limits
- block private network crawler targets
- add request rate limits
- add structured logs
- add backups for Postgres
- document restore process
- define secret rotation procedure

## Not Yet V1

- Kubernetes
- Terraform
- multi-region
- autoscaling
- billing
- enterprise SSO
