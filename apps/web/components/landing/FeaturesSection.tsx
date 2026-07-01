const FEATURES = [
  ["Grounded answers", "Responses are constrained to retrieved website content instead of loose model guesses."],
  ["Source citations", "Every answer shows the pages Perch used so visitors can verify claims."],
  ["One-line install", "A framework-free widget drops into marketing sites, docs, stores, and course pages."],
  ["Tenant isolation", "Each customer indexes and retrieves only from their own content boundary."],
  ["Domain allow-listing", "Public widget keys work only on approved domains."],
  ["Streaming responses", "Answers appear token by token so the assistant feels alive and fast."],
  ["Scheduled reindexing", "Refresh content after docs, product pages, or policies change."],
  ["Question analytics", "See what visitors ask and where your site content has gaps."],
  ["Self-host path", "The architecture is ready for customer-owned deployment later."]
];

export function FeaturesSection() {
  return (
    <section className="section" id="features">
      <div className="page-wrap">
        <p className="section-kicker">Features</p>
        <h2 className="section-title">Built around trust, not chatbot theater.</h2>
        <p className="section-copy">
          Perch is not a scripted FAQ with a nicer bubble. It is a crawler, indexer, retrieval system, and embeddable assistant packaged for one specific job.
        </p>
        <div className="features-grid">
          {FEATURES.map(([title, body], index) => (
            <article className="panel" key={title}>
              <div className="feature-icon">{index + 1}</div>
              <h3>{title}</h3>
              <p>{body}</p>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}
