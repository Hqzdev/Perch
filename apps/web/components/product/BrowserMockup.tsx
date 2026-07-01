import { ChatWidgetMock } from "@/components/product/ChatWidgetMock";

export function BrowserMockup() {
  return (
    <div className="browser" aria-label="Website preview with Perch widget">
      <div className="browser-top">
        <span className="window-dot red" />
        <span className="window-dot yellow" />
        <span className="window-dot slate" />
        <span className="address">acme-docs.com/guides/api-keys</span>
      </div>
      <div className="browser-body">
        <aside className="mock-sidebar">
          <div className="mock-lines">
            <span className="line strong" />
            <span className="line" style={{ width: "92px" }} />
            <span className="line" style={{ width: "78px", background: "#f1f2f4" }} />
            <span className="line" style={{ width: "96px" }} />
            <span className="line" style={{ width: "70px" }} />
            <span className="line" style={{ width: "86px" }} />
          </div>
        </aside>
        <section className="mock-content">
          <article className="content-card">
            <small>Security</small>
            <h3>Managing API keys</h3>
            <p>Every workspace can hold up to five active API keys. Keys are scoped to a single environment and can be rolled at any time from Settings.</p>
            <p>
              <span className="highlight">When you roll a key, the previous key remains valid for 24 hours to allow a clean migration.</span>
            </p>
            <p>Store keys as environment variables and never commit them to source control.</p>
          </article>
        </section>
        <ChatWidgetMock />
      </div>
    </div>
  );
}
