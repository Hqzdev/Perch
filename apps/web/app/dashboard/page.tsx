import Link from "next/link";

import { InstallSnippetButton } from "./InstallSnippetButton";

export const dynamic = "force-dynamic";

type SiteSummary = {
  id: string;
  organization_id: string;
  name: string;
  origin: string;
  script_key: string;
  pages_indexed: number;
  conversations_count: number;
  last_indexed_at: string | null;
  created_at: string;
};

type SiteDetail = {
  site: SiteSummary;
  install_snippet: string;
};

type PageSummary = {
  id: string;
  url: string;
  title: string | null;
  status: string;
  chunks_indexed: number;
  last_indexed_at: string | null;
};

type ConversationSummary = {
  id: string;
  visitor_id: string | null;
  messages_count: number;
  last_message_at: string | null;
  created_at: string;
};

type DashboardData = {
  connected: boolean;
  sites: SiteSummary[];
  detail: SiteDetail | null;
  pages: PageSummary[];
  conversations: ConversationSummary[];
};

const gatewayUrl = process.env.NEXT_PUBLIC_PERCH_GATEWAY_URL ?? "http://localhost:18080";

export default async function DashboardPage() {
  const data = await loadDashboardData();
  const activeSite = data.detail?.site ?? data.sites[0] ?? demoSite;
  const installSnippet =
    data.detail?.install_snippet ??
    `<script src="https://cdn.perch.ai/widget.js" data-perch-key="${activeSite.script_key}"></script>`;
  const pages = data.pages.length > 0 ? data.pages : demoPages;
  const conversations = data.conversations.length > 0 ? data.conversations : demoConversations;

  return (
    <main className="dashboard-shell">
      <aside className="dashboard-sidebar">
        <Link className="dashboard-brand" href="/">
          <span className="dashboard-brand-mark">P</span>
          <span>Perch</span>
        </Link>
        <nav className="dashboard-nav" aria-label="Dashboard">
          <a data-active="true" href="#overview">Overview</a>
          <a href="#install">Install</a>
          <a href="#pages">Pages</a>
          <a href="#conversations">Conversations</a>
        </nav>
        <div className="dashboard-sidebar-footer">
          <span>Gateway</span>
          <strong data-status={data.connected ? "live" : "offline"}>
            {data.connected ? "Connected" : "Dev preview"}
          </strong>
        </div>
      </aside>

      <section className="dashboard-main">
        <header className="dashboard-topbar">
          <div>
            <p className="dashboard-eyebrow">Site command center</p>
            <h1>{activeSite.name}</h1>
          </div>
          <a className="dashboard-primary-action" href={activeSite.origin}>
            Open site
          </a>
        </header>

        <section className="dashboard-grid" id="overview">
          <article className="dashboard-metric">
            <span>Indexed pages</span>
            <strong>{activeSite.pages_indexed}</strong>
            <small>{formatDate(activeSite.last_indexed_at) ?? "No index run yet"}</small>
          </article>
          <article className="dashboard-metric">
            <span>Conversations</span>
            <strong>{activeSite.conversations_count}</strong>
            <small>Latest visitor activity</small>
          </article>
          <article className="dashboard-metric">
            <span>Widget key</span>
            <strong>{compactKey(activeSite.script_key)}</strong>
            <small>{activeSite.origin}</small>
          </article>
        </section>

        <section className="dashboard-layout">
          <article className="dashboard-panel dashboard-install" id="install">
            <div className="dashboard-panel-header">
              <div>
                <p className="dashboard-eyebrow">Install</p>
                <h2>Widget snippet</h2>
              </div>
              <InstallSnippetButton snippet={installSnippet} />
            </div>
            <pre>{installSnippet}</pre>
            <div className="dashboard-browser-preview">
              <div className="dashboard-browser-bar">
                <span />
                <span />
                <span />
                <strong>{domainFromOrigin(activeSite.origin)}</strong>
              </div>
              <div className="dashboard-browser-body">
                <div>
                  <span />
                  <span />
                  <span />
                </div>
                <aside>
                  <strong>Ask {activeSite.name}</strong>
                  <p>Answers use indexed pages and source citations.</p>
                </aside>
              </div>
            </div>
          </article>

          <article className="dashboard-panel" id="pages">
            <div className="dashboard-panel-header">
              <div>
                <p className="dashboard-eyebrow">Knowledge base</p>
                <h2>Indexed pages</h2>
              </div>
              <span className="dashboard-pill">{pages.length} pages</span>
            </div>
            <div className="dashboard-table">
              {pages.map((page) => (
                <div className="dashboard-row" key={page.id}>
                  <div>
                    <strong>{page.title ?? "Untitled page"}</strong>
                    <span>{page.url}</span>
                  </div>
                  <small>{page.chunks_indexed} chunks</small>
                  <em>{page.status}</em>
                </div>
              ))}
            </div>
          </article>
        </section>

        <section className="dashboard-panel" id="conversations">
          <div className="dashboard-panel-header">
            <div>
              <p className="dashboard-eyebrow">Inbox</p>
              <h2>Recent conversations</h2>
            </div>
            <span className="dashboard-pill">{conversations.length} recent</span>
          </div>
          <div className="dashboard-conversations">
            {conversations.map((conversation) => (
              <article key={conversation.id}>
                <div>
                  <strong>{conversation.visitor_id ?? "anonymous visitor"}</strong>
                  <span>{conversation.messages_count} messages</span>
                </div>
                <small>{formatDate(conversation.last_message_at ?? conversation.created_at)}</small>
              </article>
            ))}
          </div>
        </section>
      </section>
    </main>
  );
}

async function loadDashboardData(): Promise<DashboardData> {
  const sites = await fetchJson<SiteSummary[]>("/v1/sites");

  if (!sites || sites.length === 0) {
    return {
      connected: Boolean(sites),
      sites: [],
      detail: null,
      pages: [],
      conversations: []
    };
  }

  const siteId = sites[0].id;
  const [detail, pages, conversations] = await Promise.all([
    fetchJson<SiteDetail>(`/v1/sites/${siteId}`),
    fetchJson<PageSummary[]>(`/v1/sites/${siteId}/pages`),
    fetchJson<ConversationSummary[]>(`/v1/sites/${siteId}/conversations`)
  ]);

  return {
    connected: true,
    sites,
    detail,
    pages: pages ?? [],
    conversations: conversations ?? []
  };
}

async function fetchJson<T>(path: string): Promise<T | null> {
  try {
    const response = await fetch(`${gatewayUrl}${path}`, {
      cache: "no-store"
    });

    if (!response.ok) {
      return null;
    }

    return response.json() as Promise<T>;
  } catch {
    return null;
  }
}

function compactKey(value: string) {
  if (value.length <= 16) {
    return value;
  }

  return `${value.slice(0, 10)}...${value.slice(-6)}`;
}

function domainFromOrigin(origin: string) {
  try {
    return new URL(origin).hostname;
  } catch {
    return origin;
  }
}

function formatDate(value: string | null) {
  if (!value) {
    return null;
  }

  return new Intl.DateTimeFormat("en", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit"
  }).format(new Date(value));
}

const demoSite: SiteSummary = {
  id: "demo",
  organization_id: "demo",
  name: "Perch Demo",
  origin: "https://portfolio-demo.perch.local",
  script_key: "pk_dev_dashboard_preview",
  pages_indexed: 0,
  conversations_count: 0,
  last_indexed_at: null,
  created_at: new Date(0).toISOString()
};

const demoPages: PageSummary[] = [
  {
    id: "demo-page",
    url: "https://portfolio-demo.perch.local/docs/install",
    title: "Install Perch",
    status: "preview",
    chunks_indexed: 0,
    last_indexed_at: null
  }
];

const demoConversations: ConversationSummary[] = [
  {
    id: "demo-conversation",
    visitor_id: "portfolio-preview",
    messages_count: 0,
    last_message_at: null,
    created_at: new Date(0).toISOString()
  }
];
