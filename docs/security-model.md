# Security Model

Perch has three primary trust boundaries:

- customer dashboard users
- public website visitors
- backend service infrastructure

The most important security rule is that a public widget key must never become authorization to access another tenant's content or configuration.

## Assets

- tenant records
- site records
- indexed page metadata
- vector chunks
- raw HTML snapshots when enabled
- chat messages
- usage events
- provider API keys
- dashboard sessions
- public widget keys

## Threats

### Cross-Tenant Retrieval

A visitor or compromised widget must not retrieve chunks from another tenant.

Required controls:

- server-side tenant resolution from public key
- tenant filter applied in retrieval
- no trusted tenant IDs from browser input
- tests for wrong-tenant queries

### Unauthorized Embed

A public widget key copied from one site must not work on another domain.

Required controls:

- origin validation
- allow-listed domains per site
- clear failure response
- rate limit failed origin checks

### Crawler SSRF

The crawler must not fetch private network targets or cloud metadata endpoints.

Required controls:

- URL scheme allow-list
- private IP blocking
- redirect validation
- request timeout
- response size limit
- crawl page limit

### Prompt Injection

Website content can contain hostile instructions. Retrieval and answer generation must treat page content as untrusted evidence, not authority.

Required controls:

- system prompt separating instructions from evidence
- answer only from retrieved content
- cite sources
- refuse unsupported claims

### Widget XSS

Assistant output and citations render inside customer websites.

Required controls:

- escape rendered text
- validate citation URLs
- avoid unsafe HTML injection
- isolate widget styles

## Security Review Checklist

- Does this change affect tenant isolation?
- Does it trust browser-provided tenant or site IDs?
- Does it add a new crawler fetch path?
- Does it render model output as HTML?
- Does it expose configuration through a public endpoint?
- Does it add a new secret?
- Does it change provider request payloads?
