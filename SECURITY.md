# Security Policy

Perch embeds on customer websites, crawls customer content, and answers visitor questions from indexed sources. Security issues are treated as product-critical.

## Supported Versions

Perch is pre-release. Security fixes target the main branch until the project publishes versioned releases.

## Reporting a Vulnerability

Use GitHub private vulnerability reporting when available. If private reporting is not available, open a minimal public issue that says a security report exists but does not include exploit details.

Include:

- affected component
- reproduction steps
- expected impact
- affected commit or version
- any relevant logs with secrets removed

Do not include real API keys, session tokens, private customer content, or credentials.

## Security Scope

High-impact areas:

- tenant isolation failures
- cross-tenant retrieval
- widget key misuse
- missing domain allow-list enforcement
- crawler SSRF
- stored or reflected XSS in widget or dashboard
- prompt injection that causes source boundary bypass
- leakage of indexed private content
- unsafe handling of provider API keys
- unauthenticated administrative actions

## Response Expectations

The project is early-stage and does not promise a formal SLA. Valid reports should receive acknowledgement as soon as a maintainer is available.

## Safe Harbor

Good-faith security research is welcome when it avoids data destruction, service disruption, credential theft, persistence, and access to data that does not belong to the reporter.
