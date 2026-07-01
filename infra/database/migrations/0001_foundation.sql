create extension if not exists pgcrypto;

create table if not exists organizations (
    id uuid primary key default gen_random_uuid(),
    name text not null,
    created_at timestamptz not null default now()
);

create table if not exists sites (
    id uuid primary key default gen_random_uuid(),
    organization_id uuid not null references organizations(id) on delete cascade,
    name text not null,
    origin text not null,
    script_key text not null unique,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (organization_id, origin)
);

create table if not exists site_pages (
    id uuid primary key default gen_random_uuid(),
    site_id uuid not null references sites(id) on delete cascade,
    url text not null,
    canonical_url text,
    title text,
    content_hash text,
    status text not null default 'pending',
    last_indexed_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (site_id, url)
);

create table if not exists page_chunks (
    id uuid primary key default gen_random_uuid(),
    page_id uuid not null references site_pages(id) on delete cascade,
    chunk_index integer not null,
    content text not null,
    token_count integer not null default 0,
    source_url text not null,
    source_title text,
    created_at timestamptz not null default now(),
    unique (page_id, chunk_index)
);

create table if not exists conversations (
    id uuid primary key default gen_random_uuid(),
    site_id uuid not null references sites(id) on delete cascade,
    visitor_id text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists messages (
    id uuid primary key default gen_random_uuid(),
    conversation_id uuid not null references conversations(id) on delete cascade,
    role text not null check (role in ('visitor', 'assistant', 'system')),
    content text not null,
    citations jsonb not null default '[]'::jsonb,
    created_at timestamptz not null default now()
);

create index if not exists sites_organization_id_idx on sites(organization_id);
create index if not exists site_pages_site_id_idx on site_pages(site_id);
create index if not exists site_pages_status_idx on site_pages(status);
create index if not exists page_chunks_page_id_idx on page_chunks(page_id);
create index if not exists conversations_site_id_idx on conversations(site_id);
create index if not exists messages_conversation_id_idx on messages(conversation_id);
