# Governance

Perch is currently maintained as a single-maintainer project.

## Decision Making

Architecture and product decisions should optimize for:

1. shipping a working cited-answer website assistant
2. preserving tenant isolation and domain boundaries
3. keeping the Rust backend understandable
4. avoiding premature infrastructure complexity
5. making the demo credible to customers and reviewers

## Architecture Changes

Large architecture changes should be documented before implementation when they affect:

- service boundaries
- database ownership
- tenant isolation
- crawler behavior
- retrieval and citation behavior
- widget public API
- deployment topology

Use `docs/architecture.md` for accepted decisions until a formal RFC process is needed.

## Maintainer Responsibilities

Maintainers should:

- keep the roadmap honest
- reject fake production claims
- preserve a small V1 scope
- require security review for public widget and tenant changes
- avoid adding dependencies without a concrete reason
