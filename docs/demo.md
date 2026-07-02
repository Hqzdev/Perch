# Demo Flow

Run the local stack:

```sh
docker compose up --build -d
```

Run the portfolio demo:

```sh
./scripts/portfolio-demo.sh
```

The script creates a site, indexes one source page through Gateway, asks the widget chat endpoint a question, and prints the grounded answer plus citations.

Expected flow:

```txt
script
  -> gateway /v1/sites
  -> gateway /v1/sites/:siteId/pages
  -> indexer /v1/index/pages
  -> Postgres page_chunks
  -> Qdrant perch_chunks vectors
  -> gateway /v1/widget/chat
  -> retrieval /v1/answer
  -> Qdrant vector search
  -> deterministic answer generator
  -> cited answer
```

The local demo uses deterministic hash embeddings and keeps `PERCH_LLM_PROVIDER=disabled`, so Qdrant retrieval works without external API keys. Set `PERCH_LLM_PROVIDER=openai`, `PERCH_LLM_API_KEY`, and `PERCH_LLM_MODEL` to let retrieval generate the final answer with an OpenAI-compatible chat completion API. The crawl job API is implemented separately for single-page fetches, while queue-backed crawling remains future work.
