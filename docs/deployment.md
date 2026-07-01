# Deployment

Perch does not have a production deployment target yet. This document defines the intended deployment shape so implementation does not drift.

## V1 Deployment

V1 should use a simple Docker Compose or small VPS deployment:

- gateway container
- indexer container
- retrieval container
- Postgres
- Redis
- Qdrant
- web app

This is enough for a real demo and avoids premature Kubernetes work.

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
