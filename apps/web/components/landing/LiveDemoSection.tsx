import { ChatWidgetMock } from "@/components/product/ChatWidgetMock";

export function LiveDemoSection() {
  return (
    <section className="section" id="demo">
      <div className="page-wrap">
        <p className="section-kicker">See it answer</p>
        <h2 className="section-title">A real question, a cited answer.</h2>
        <p className="section-copy">
          Perch retrieves relevant passages from indexed pages, answers in the site owner&apos;s voice, and shows exactly where each claim came from.
        </p>
        <div className="two-col" style={{ marginTop: "38px" }}>
          <div className="browser">
            <div className="browser-top">
              <span className="window-dot red" />
              <span className="window-dot yellow" />
              <span className="window-dot slate" />
              <span className="address">northwind.io/docs/security</span>
            </div>
            <div className="mock-content">
              <article className="content-card">
                <small>Security</small>
                <h3>Managing API keys</h3>
                <p>Every workspace can hold up to five active API keys. Keys are scoped to a single environment and can be rolled at any time.</p>
                <p>
                  <span className="highlight">When you roll a key, the previous key remains valid for 24 hours to allow a clean migration.</span>
                </p>
                <p>After that window it is revoked automatically.</p>
              </article>
            </div>
          </div>
          <ChatWidgetMock dark />
        </div>
      </div>
    </section>
  );
}
