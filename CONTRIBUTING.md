# Contributing to Perch

Perch is an early-stage product repository. Contributions should make the product clearer, safer, easier to run, or closer to the Tier A demo.

## Priorities

Work should follow this order:

1. Make the website and widget demo polished and honest.
2. Implement the Rust service skeleton with clear boundaries.
3. Build the crawl to index to retrieve path.
4. Add tenant isolation and domain allow-listing.
5. Add tests and CI coverage around real behavior.

Do not add broad platform features before the core website assistant flow works end to end.

## Development Setup

Web app:

```sh
cd apps/web
npm install
npm run dev
```

Production build:

```sh
cd apps/web
npm run build
```

Rust services are scaffolded by directory boundary. Add workspace manifests before adding service code.

## Code Standards

- No comments in code.
- Keep code self-documenting through naming and structure.
- Follow single responsibility.
- Prefer composition over inheritance.
- Keep modules small and intentional.
- Avoid dead code, placeholders, and fake abstractions.
- Do not mix product copy from unrelated projects into Perch.

## Architecture Rules

- Domain code must not depend on HTTP, SQL, Redis, Qdrant, model providers, or framework APIs.
- Application code coordinates use cases and depends on ports, not concrete infrastructure clients.
- Infrastructure code owns external systems.
- Interfaces own HTTP and queue entrypoints.
- Shared crates must stay narrow and must not become dumping grounds.

## Pull Requests

Before opening a pull request:

- Run the relevant build or test command.
- Keep changes scoped to one purpose.
- Update documentation when changing architecture, setup, security boundaries, or public behavior.
- Explain tradeoffs in the PR description.
- Include screenshots for visible web or widget changes.

## Commit Style

Use clear imperative commits:

```txt
Add Perch landing page sections
Create gateway service skeleton
Document tenant isolation boundary
```

Avoid vague commits such as:

```txt
updates
fix stuff
wip
```
