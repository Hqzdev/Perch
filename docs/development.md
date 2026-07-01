# Development

## Requirements

- Node.js 20 or newer
- npm
- Rust toolchain when backend manifests are added
- Docker when local infrastructure is added

## Web App

Install dependencies:

```sh
cd apps/web
npm install
```

Run locally:

```sh
npm run dev
```

Build:

```sh
npm run build
```

## Backend

The backend directories are scaffolded, but Rust manifests are not committed yet. Add the workspace manifest and service manifests before implementing backend code.

## Environment

Start from `.env.example` files and keep secrets out of Git.

Never commit:

- provider API keys
- database URLs with passwords
- session secrets
- production widget keys
- crawl targets containing private customer data

## Quality Bar

Before merging visible UI work:

- build the app
- verify desktop and mobile layout
- check that product copy says Perch, not another project
- include screenshots in the pull request

Before merging backend work:

- run formatting
- run tests
- document new environment variables
- update architecture docs when boundaries change
