create table if not exists crawl_jobs (
    id uuid primary key default gen_random_uuid(),
    site_id uuid not null references sites(id) on delete cascade,
    target_url text not null,
    status text not null default 'pending' check (status in ('pending', 'running', 'succeeded', 'failed')),
    page_id uuid,
    pages_indexed integer not null default 0,
    chunks_indexed integer not null default 0,
    error_message text,
    started_at timestamptz,
    finished_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index if not exists crawl_jobs_site_id_idx on crawl_jobs(site_id);
create index if not exists crawl_jobs_status_idx on crawl_jobs(status);

alter table crawl_jobs add column if not exists page_id uuid;
