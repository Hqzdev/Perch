export function DashboardMockup() {
  return (
    <div className="dashboard">
      <div className="dashboard-head">
        <div>
          <h3>acme-docs.com</h3>
          <p className="section-copy" style={{ marginTop: "6px", fontSize: "14px" }}>
            Last indexed 4 minutes ago
          </p>
        </div>
        <a className="btn btn-primary" href="#how">
          Reindex
        </a>
      </div>
      <div className="dashboard-grid">
        <div className="metric-stack">
          <div className="metric">
            <span>Indexed pages</span>
            <strong>384</strong>
          </div>
          <div className="metric">
            <span>Answer coverage</span>
            <strong>91%</strong>
          </div>
          <div className="snippet">&lt;script src=&quot;https://cdn.perch.ai/widget.js&quot; data-perch-key=&quot;pk_live_8f2c...a91&quot;&gt;&lt;/script&gt;</div>
        </div>
        <div className="activity">
          <div className="activity-row">
            <strong>How do I rotate an API key?</strong>
            <span>Answered with 2 cited sources</span>
          </div>
          <div className="activity-row">
            <strong>Does the enterprise plan include SSO?</strong>
            <span>Answered with 3 cited sources</span>
          </div>
          <div className="activity-row">
            <strong>Can I export audit logs?</strong>
            <span>Needs source coverage review</span>
          </div>
          <div className="activity-row">
            <strong>Theme</strong>
            <span>Green accent, bottom-right bubble, source links enabled</span>
          </div>
        </div>
      </div>
    </div>
  );
}
